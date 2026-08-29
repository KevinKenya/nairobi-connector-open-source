// Copyright 2026 Kevin Chege
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// File: crates/nairobi-axum-refinery/src/analyze.rs
// Author: Kevin Chege. Location: Nairobi
// Date: 2026-05-21

// nairobi-open-source-release/crates/nairobi-axum-refinery/src/analyze.rs
use memmap2::Mmap;
use nairobi_protocol::{
    CleanDataStrategy, CorrelationResult, DistilledAnalytics, FusedAnalyticsResult,
    SchemaInspection,
};
use nairobi_protocol::{ImperialError, ImperialResult};
use polars::prelude::*;
use rayon::prelude::*;
use std::os::unix::io::FromRawFd;
use std::sync::Arc;
use tracing::info;
use zbus::zvariant::OwnedFd;

/// Get Peak RSS (Resident Set Size) from /proc/self/status
pub fn get_peak_rss() -> u64 {
    if let Ok(status) = std::fs::read_to_string("/proc/self/status") {
        for line in status.lines() {
            if line.starts_with("VmHWM:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    if let Ok(kb) = parts[1].parse::<u64>() {
                        return kb * 1024;
                    }
                }
            }
        }
    }
    0
}

/// Log compute density audit to telemetry
fn log_density_audit(peak_rss: u64) {
    use std::io::Write;
    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    let log_path = std::path::PathBuf::from(home)
        .join("automation_telemetry.log");
    if let Ok(mut file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)
    {
        let _ = writeln!(file, "[DENSITY_AUDIT] Peak RSS: {} bytes", peak_rss);
    }
}

/// Compute Spearman rank correlation manually.
///
/// Algorithm:
/// 1. Collect paired non-null values
/// 2. Rank each variable independently (average rank for ties)
/// 3. Compute Pearson correlation on the ranks
///
/// This is a self-contained implementation that doesn't depend on
/// potentially missing Polars feature flags.
fn compute_spearman_rank_corr(ca1: &Float64Chunked, ca2: &Float64Chunked) -> f64 {
    // 1. Collect paired non-null observations
    let pairs: Vec<(f64, f64)> = ca1
        .into_iter()
        .zip(ca2.into_iter())
        .filter_map(|(a, b)| match (a, b) {
            (Some(a), Some(b)) => Some((a, b)),
            _ => None,
        })
        .collect();

    let n = pairs.len();
    if n < 2 {
        return 0.0;
    }

    // 2. Compute average ranks (handles ties correctly)
    let x_ranks = compute_ranks(&pairs.iter().map(|(x, _)| *x).collect::<Vec<f64>>());
    let y_ranks = compute_ranks(&pairs.iter().map(|(_, y)| *y).collect::<Vec<f64>>());

    // 3. Pearson correlation on ranks
    let n_f = n as f64;
    let mean_rx: f64 = x_ranks.iter().sum::<f64>() / n_f;
    let mean_ry: f64 = y_ranks.iter().sum::<f64>() / n_f;

    let mut cov = 0.0_f64;
    let mut var_x = 0.0_f64;
    let mut var_y = 0.0_f64;

    for i in 0..n {
        let dx = x_ranks[i] - mean_rx;
        let dy = y_ranks[i] - mean_ry;
        cov += dx * dy;
        var_x += dx * dx;
        var_y += dy * dy;
    }

    let denom = (var_x * var_y).sqrt();
    if denom == 0.0 {
        0.0
    } else {
        cov / denom
    }
}

/// Compute average ranks for a vector of values.
/// Tied values receive the mean of their ordinal ranks.
fn compute_ranks(values: &[f64]) -> Vec<f64> {
    let n = values.len();
    let mut indexed: Vec<(usize, f64)> = values.iter().copied().enumerate().collect();
    indexed.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

    let mut ranks = vec![0.0_f64; n];
    let mut i = 0;
    while i < n {
        // Find the extent of the tie group
        let mut j = i + 1;
        while j < n && (indexed[j].1 - indexed[i].1).abs() < f64::EPSILON {
            j += 1;
        }
        // Average rank for the tie group (1-indexed)
        let avg_rank = (i + 1 + j) as f64 / 2.0;
        for k in i..j {
            ranks[indexed[k].0] = avg_rank;
        }
        i = j;
    }

    ranks
}

/// Compute Pearson correlation manually from two Float64Chunked arrays.
///
/// Uses the standard formula: r = Σ((xi-x̄)(yi-ȳ)) / sqrt(Σ(xi-x̄)² * Σ(yi-ȳ)²)
/// Returns 0.0 if either series has zero variance.
fn compute_pearson_corr(ca1: &Float64Chunked, ca2: &Float64Chunked) -> f64 {
    let pairs: Vec<(f64, f64)> = ca1
        .into_iter()
        .zip(ca2.into_iter())
        .filter_map(|(a, b)| match (a, b) {
            (Some(a), Some(b)) => Some((a, b)),
            _ => None,
        })
        .collect();

    let n = pairs.len();
    if n < 2 {
        return 0.0;
    }

    let n_f = n as f64;
    let mean_x: f64 = pairs.iter().map(|(x, _)| *x).sum::<f64>() / n_f;
    let mean_y: f64 = pairs.iter().map(|(_, y)| *y).sum::<f64>() / n_f;

    let mut cov = 0.0_f64;
    let mut var_x = 0.0_f64;
    let mut var_y = 0.0_f64;

    for (x, y) in &pairs {
        let dx = x - mean_x;
        let dy = y - mean_y;
        cov += dx * dy;
        var_x += dx * dx;
        var_y += dy * dy;
    }

    let denom = (var_x * var_y).sqrt();
    if denom == 0.0 {
        0.0
    } else {
        cov / denom
    }
}



/// Safe wrapper for memory-mapping an OwnedFd.
/// Automatically handles the AsRawFd -> Mmap transition.
pub struct SafeMmap {
    mmap: Mmap,
}

impl SafeMmap {
    /// Map an OwnedFd read-only.
    pub fn map(handle: &OwnedFd) -> ImperialResult<Self> {
        let mmap = unsafe {
            Mmap::map(handle).map_err(|e| ImperialError::Analysis(format!("Mmap failed: {}", e)))?
        };
        Ok(Self { mmap })
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.mmap[..]
    }
}

/// Consolidated statistical profile for analytical results.
pub struct StatisticalProfile {
    pub min: f64,
    pub max: f64,
    pub mean: f64,
    pub std_dev: f64,
    pub variance: f64,
    pub p95: f64,
    pub p99: f64,
    pub skewness: f64,
    pub kurtosis: f64,
    pub anomalies: Vec<String>,
}

impl StatisticalProfile {
    /// Compute the statistical profile from a Float64 series.
    pub fn compute(
        ca: &Float64Chunked,
        df: &DataFrame,
        total_rows: u64,
    ) -> ImperialResult<Self> {
        let mut profile = Self {
            min: 0.0,
            max: 0.0,
            mean: 0.0,
            std_dev: 0.0,
            variance: 0.0,
            p95: 0.0,
            p99: 0.0,
            skewness: 0.0,
            kurtosis: 0.0,
            anomalies: Vec::new(),
        };

        if total_rows > 0 {
            profile.min = ca.min().unwrap_or(0.0);
            profile.max = ca.max().unwrap_or(0.0);
            profile.mean = ca.mean().unwrap_or(0.0);

            if total_rows >= 2 {
                profile.std_dev = ca.std(1).unwrap_or(0.0);
                profile.variance = ca.var(1).unwrap_or(0.0);

                if profile.std_dev > 0.0 {
                    let mean = profile.mean;
                    let values: Vec<f64> = ca.into_no_null_iter().collect();
                    let (sum_cube, sum_fourth) = values
                        .par_iter()
                        .map(|v| {
                            let diff = v - mean;
                            (diff.powi(3), diff.powi(4))
                        })
                        .reduce(|| (0.0_f64, 0.0_f64), |a, b| (a.0 + b.0, a.1 + b.1));
                    
                    let n = total_rows as f64;
                    profile.skewness = (sum_cube / n) / profile.std_dev.powi(3);
                    profile.kurtosis = (sum_fourth / n) / profile.std_dev.powi(4) - 3.0;

                    // Anomaly detection
                    let mut anomaly_indices: Vec<usize> = Vec::new();
                    for (i, opt_v) in ca.into_iter().enumerate() {
                        if let Some(val) = opt_v {
                            if (val - mean).abs() / profile.std_dev > 3.0 {
                                anomaly_indices.push(i);
                            }
                        }
                    }
                    let limit = std::cmp::min(anomaly_indices.len(), 5);
                    for &idx in &anomaly_indices[..limit] {
                        if let Some(row) = df.get(idx) {
                            let row_str: String = row
                                .iter()
                                .map(|v: &AnyValue<'_>| v.to_string())
                                .collect::<Vec<String>>()
                                .join(",");
                            profile.anomalies.push(row_str);
                        }
                    }
                }
            }

            profile.p95 = ca
                .quantile(0.95, QuantileInterpolOptions::Linear)
                .map_err(|e| ImperialError::Analysis(format!("P95 calculation failed: {}", e)))?
                .unwrap_or(0.0);
            profile.p99 = ca
                .quantile(0.99, QuantileInterpolOptions::Linear)
                .map_err(|e| ImperialError::Analysis(format!("P99 calculation failed: {}", e)))?
                .unwrap_or(0.0);
        }

        Ok(profile)
    }
}

/// Vectorized Analytics Engine using Polars.
pub struct AnalyzeEngine {
    thread_pool: Arc<rayon::ThreadPool>,
}

impl AnalyzeEngine {
    pub fn new() -> ImperialResult<Self> {
        // Govern threads: use half of available parallelism to avoid host starvation.
        let threads = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1) / 2;
        let threads = std::cmp::max(1, threads);

        info!("[ANALYZE] Initializing Rayon pool with {} threads", threads);

        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build()
            .map_err(|e| ImperialError::Analysis(format!("Failed to build Rayon pool: {}", e)))?;

        Ok(Self {
            thread_pool: Arc::new(pool),
        })
    }

    /// Executes vectorized analytics on a memfd buffer.
    pub fn analyze(&self, handle: OwnedFd, query: &str) -> ImperialResult<DistilledAnalytics> {
        // 1. Map the handle read-only via SafeMmap
        let smmap = SafeMmap::map(&handle)?;

        // 2. Execute within the capped Rayon pool
        self.thread_pool.install(|| {
            let cursor = std::io::Cursor::new(smmap.as_slice());

            // Schema override for the target column
            let schema_override = Schema::from_iter(vec![Field::new(query, DataType::Float64)]);

            let df = CsvReader::new(cursor)
                .has_header(true)
                .with_dtypes(Some(Arc::new(schema_override)))
                .with_ignore_errors(true)
                .finish()
                .map_err(|e| ImperialError::Analysis(format!("Polars read failed: {}", e)))?;

            let series: &Series = df.column(query).map_err(|e| {
                ImperialError::Analysis(format!("Column '{}' not found: {}", query, e))
            })?;

            let casted: Series = series
                .cast(&DataType::Float64)
                .map_err(|e| ImperialError::Analysis(format!("Cast failed: {}", e)))?;

            let ca: &Float64Chunked = casted
                .f64()
                .map_err(|e| ImperialError::Analysis(format!("Not a Float64 column: {}", e)))?;

            let total_rows = df.height() as u64;

            // 3. Compute Statistical Profile (DRY logic)
            let profile = StatisticalProfile::compute(ca, &df, total_rows)?;

            Ok(DistilledAnalytics {
                total_rows,
                min: profile.min,
                max: profile.max,
                mean: profile.mean,
                std_dev: profile.std_dev,
                variance: profile.variance,
                p95: profile.p95,
                p99: profile.p99,
                skewness: profile.skewness,
                kurtosis: profile.kurtosis,
                handle,
                anomalies: profile.anomalies,
            })
        })
    }

    /// Inspects the schema and returns a sample of the data.
    pub fn inspect_schema(&self, handle: OwnedFd) -> ImperialResult<SchemaInspection> {
        let smmap = SafeMmap::map(&handle)?;

        self.thread_pool.install(|| {
            let cursor = std::io::Cursor::new(smmap.as_slice());

            let df = CsvReader::new(cursor)
                .has_header(true)
                .infer_schema(Some(100))
                .with_n_rows(Some(100))
                .with_ignore_errors(true)
                .finish()
                .map_err(|e| ImperialError::Analysis(format!("Polars read failed: {}", e)))?;

            let schema = df.schema();
            let columns: Vec<String> = schema.iter_names().map(|s: &_| s.to_string()).collect();
            let data_types: Vec<String> = schema
                .iter_dtypes()
                .map(|d: &DataType| d.to_string())
                .collect();

            let mut null_counts: Vec<u64> = Vec::new();
            for col_name in &columns {
                let count: u64 = match df.column(col_name) {
                    Ok(s) => s.null_count() as u64,
                    Err(_) => 0,
                };
                null_counts.push(count);
            }

            let sample_df = df.head(Some(5));
            let sample_json = format!("{:?}", sample_df);

            Ok(SchemaInspection {
                columns,
                data_types,
                null_counts,
                sample_json,
                handle,
            })
        })
    }

    /// Cleans the data using specified strategies.
    pub fn clean_data(
        &self,
        handle: OwnedFd,
        strategies: Vec<CleanDataStrategy>,
    ) -> ImperialResult<OwnedFd> {
        let smmap = SafeMmap::map(&handle)?;

        self.thread_pool.install(|| {
            let cursor = std::io::Cursor::new(smmap.as_slice());

            let mut df = CsvReader::new(cursor)
                .has_header(true)
                .infer_schema(Some(1000))
                .with_ignore_errors(true)
                .finish()
                .map_err(|e| ImperialError::Analysis(format!("Polars read failed: {}", e)))?;

            // Apply cleaning strategies
            for strat in strategies {
                match strat.strategy.as_str() {
                    "drop_column" => {
                        let _ = df.drop_in_place(&strat.column);
                    }
                    "drop_row" => {
                        df = df
                            .drop_nulls::<String>(Some(&[strat.column]))
                            .map_err(|e| {
                                ImperialError::Analysis(format!("Drop nulls failed: {}", e))
                            })?;
                    }
                    "fill_zero" => {
                        if let Ok(s) = df.column(&strat.column) {
                            let filled: Series = s
                                .fill_null(FillNullStrategy::Zero)
                                .unwrap_or_else(|_| s.clone());
                            let _ = df.replace(&strat.column, filled);
                        }
                    }
                    _ => {}
                }
            }

            // Write out to a new memfd
            let opts = memfd::MemfdOptions::default().allow_sealing(true);
            let memfd = opts
                .create("cleaned_data.csv")
                .map_err(|e| ImperialError::Analysis(format!("Failed to create memfd: {}", e)))?;

            let mut out_file = memfd.into_file();
            CsvWriter::new(&mut out_file)
                .has_header(true)
                .finish(&mut df)
                .map_err(|e| ImperialError::Analysis(format!("Failed to write CSV: {}", e)))?;

            use std::os::fd::IntoRawFd;
            let raw_fd = out_file.into_raw_fd();
            let new_handle = unsafe { OwnedFd::from_raw_fd(raw_fd) };

            Ok(new_handle)
        })
    }

    /// Execute a SQL query on a memfd buffer and return a new memfd buffer.
    pub async fn sql_query(&self, handle: OwnedFd, query: &str) -> ImperialResult<OwnedFd> {
        let query_owned = query.to_string();
        let pool = self.thread_pool.clone();

        let result = tokio::time::timeout(
            std::time::Duration::from_secs(10),
            tokio::task::spawn_blocking(move || {
                let smmap = SafeMmap::map(&handle)?;

                pool.install(|| {
                    let cursor = std::io::Cursor::new(smmap.as_slice());

                    let df = CsvReader::new(cursor)
                        .has_header(true)
                        .infer_schema(Some(1000))
                        .with_ignore_errors(true)
                        .finish()
                        .map_err(|e| {
                            ImperialError::Analysis(format!("Polars read failed: {}", e))
                        })?;

                    let mut ctx = polars_sql::SQLContext::new();
                    ctx.register("dataset", df.lazy());

                    let result_lf = ctx.execute(&query_owned).map_err(|e| {
                        ImperialError::Analysis(format!("SQL compilation failed: {}", e))
                    })?;

                    let mut result_df = result_lf.collect().map_err(|e| {
                        ImperialError::Analysis(format!("SQL execution failed: {}", e))
                    })?;

                    // Write out to a new memfd
                    let opts = memfd::MemfdOptions::default().allow_sealing(true);
                    let memfd = opts.create("sql_result.csv").map_err(|e| {
                        ImperialError::Analysis(format!("Failed to create memfd: {}", e))
                    })?;

                    let mut out_file = memfd.into_file();
                    CsvWriter::new(&mut out_file)
                        .has_header(true)
                        .finish(&mut result_df)
                        .map_err(|e| {
                            ImperialError::Analysis(format!("Failed to write CSV: {}", e))
                        })?;

                    use std::os::fd::IntoRawFd;
                    let raw_fd = out_file.into_raw_fd();
                    let new_handle = unsafe { OwnedFd::from_raw_fd(raw_fd) };

                    Ok(new_handle)
                })
            }),
        )
        .await;

        match result {
            Ok(Ok(Ok(fd))) => Ok(fd),
            Ok(Ok(Err(e))) => Err(e),
            Ok(Err(e)) => Err(ImperialError::Analysis(format!(
                "Task joined failed: {}",
                e
            ))),
            Err(_) => Err(ImperialError::SystemicSeizure(
                "SQL Query Timeout: Complexity limit exceeded".to_string(),
            )),
        }
    }

    /// Calculate correlation between two columns in a memfd buffer.
    /// Returns (Pearson, Spearman) coefficients.
    pub fn correlation(&self, handle: OwnedFd, query: &str) -> ImperialResult<CorrelationResult> {
        let smmap = SafeMmap::map(&handle)?;

        let result = self.thread_pool.install(|| {
            let cursor = std::io::Cursor::new(smmap.as_slice());
            let df = CsvReader::new(cursor)
                .has_header(true)
                .with_ignore_errors(true)
                .finish()
                .map_err(|e| ImperialError::Analysis(format!("Polars read failed: {}", e)))?;

            let cols: Vec<&str> = query.split(',').map(|s| s.trim()).collect();
            if cols.len() != 2 {
                return Err(ImperialError::Analysis(
                    "Correlation requires exactly 2 column names separated by comma".to_string(),
                ));
            }

            let s1: Series = df
                .column(cols[0])
                .map_err(|e| {
                    ImperialError::Analysis(format!("Column '{}' not found: {}", cols[0], e))
                })?
                .cast(&DataType::Float64)
                .map_err(|e| {
                    ImperialError::Analysis(format!("Cast failed for '{}': {}", cols[0], e))
                })?;

            let s2: Series = df
                .column(cols[1])
                .map_err(|e| {
                    ImperialError::Analysis(format!("Column '{}' not found: {}", cols[1], e))
                })?
                .cast(&DataType::Float64)
                .map_err(|e| {
                    ImperialError::Analysis(format!("Cast failed for '{}': {}", cols[1], e))
                })?;

            // Check for zero variance (guard against NaN)
            let ca1: &Float64Chunked = s1
                .f64()
                .map_err(|e| ImperialError::Analysis(format!("Not Float64: {}", e)))?;
            let ca2: &Float64Chunked = s2
                .f64()
                .map_err(|e| ImperialError::Analysis(format!("Not Float64: {}", e)))?;

            let var1 = ca1.var(1).unwrap_or(0.0);
            let var2 = ca2.var(1).unwrap_or(0.0);

            // Guard: zero variance returns (0.0, 0.0)
            if var1 == 0.0 || var2 == 0.0 {
                return Ok(CorrelationResult {
                    pearson: 0.0,
                    spearman: 0.0,
                });
            }

            // Pearson correlation — direct formula computation
            let pearson = compute_pearson_corr(ca1, ca2);

            // Spearman rank correlation — manual rank-and-correlate
            let spearman = compute_spearman_rank_corr(ca1, ca2);

            Ok(CorrelationResult { pearson, spearman })
        });

        // Log compute density audit
        let peak_rss = get_peak_rss();
        log_density_audit(peak_rss);

        result
    }

    /// Fused crunch + correlate: single CSV parse, single D-Bus round trip.
    pub fn crunch_and_correlate(
        &self,
        handle: OwnedFd,
        column: &str,
        corr_columns: &str,
    ) -> ImperialResult<FusedAnalyticsResult> {
        let smmap = SafeMmap::map(&handle)?;

        let result = self.thread_pool.install(|| {
            let cursor = std::io::Cursor::new(smmap.as_slice());

            let schema_override = Schema::from_iter(vec![Field::new(column, DataType::Float64)]);

            let df = CsvReader::new(cursor)
                .has_header(true)
                .with_dtypes(Some(Arc::new(schema_override)))
                .with_ignore_errors(true)
                .finish()
                .map_err(|e| ImperialError::Analysis(format!("Polars read failed: {}", e)))?;

            let total_rows = df.height() as u64;

            // ── Analytics (using Centralized Statistical Cortex) ──
            let series = df.column(column).map_err(|e| {
                ImperialError::Analysis(format!("Column '{}' not found: {}", column, e))
            })?;
            let casted = series
                .cast(&DataType::Float64)
                .map_err(|e| ImperialError::Analysis(format!("Cast failed: {}", e)))?;
            let ca = casted
                .f64()
                .map_err(|e| ImperialError::Analysis(format!("Not Float64: {}", e)))?;

            let profile = StatisticalProfile::compute(ca, &df, total_rows)?;

            // ── Correlation (same DataFrame, no re-parse) ──
            let cols: Vec<&str> = corr_columns.split(',').map(|s| s.trim()).collect();
            let (pearson, spearman) = if cols.len() == 2 {
                let s1 = df.column(cols[0]).map_err(|e| {
                    ImperialError::Analysis(format!("Column '{}' not found: {}", cols[0], e))
                })?.cast(&DataType::Float64).map_err(|e| {
                    ImperialError::Analysis(format!("Cast failed: {}", e))
                })?;
                let s2 = df.column(cols[1]).map_err(|e| {
                    ImperialError::Analysis(format!("Column '{}' not found: {}", cols[1], e))
                })?.cast(&DataType::Float64).map_err(|e| {
                    ImperialError::Analysis(format!("Cast failed: {}", e))
                })?;

                let ca1 = s1.f64().map_err(|e| ImperialError::Analysis(format!("Not Float64: {}", e)))?;
                let ca2 = s2.f64().map_err(|e| ImperialError::Analysis(format!("Not Float64: {}", e)))?;

                let var1 = ca1.var(1).unwrap_or(0.0);
                let var2 = ca2.var(1).unwrap_or(0.0);

                if var1 == 0.0 || var2 == 0.0 {
                    (0.0, 0.0)
                } else {
                    // Run Pearson and Spearman concurrently
                    rayon::join(
                        || compute_pearson_corr(ca1, ca2),
                        || compute_spearman_rank_corr(ca1, ca2),
                    )
                }
            } else {
                (0.0, 0.0)
            };

            Ok(FusedAnalyticsResult {
                total_rows,
                min: profile.min,
                max: profile.max,
                mean: profile.mean,
                std_dev: profile.std_dev,
                variance: profile.variance,
                p95: profile.p95,
                p99: profile.p99,
                skewness: profile.skewness,
                kurtosis: profile.kurtosis,
                pearson,
                spearman,
                anomalies: profile.anomalies,
            })
        });

        let peak_rss = get_peak_rss();
        log_density_audit(peak_rss);

        result
    }
}
