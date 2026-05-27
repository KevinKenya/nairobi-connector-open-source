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

// crates/nairobi-axum-refinery/examples/nba_crunch.rs
// Author: Kevin Chege. Location: Nairobi
// Date: 2026-05-21

use nairobi_axum_refinery::analyze::{get_peak_rss, AnalyzeEngine};
use nairobi_protocol::MemoryPipe;
use std::fs::File;
use std::io::Write;
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::path::PathBuf;
use std::time::Instant;
use zbus::zvariant::OwnedFd;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_total = Instant::now();

    let workspace_root = std::env::var("CARGO_MANIFEST_DIR")
        .map(|p| PathBuf::from(p).parent().map(|p| p.parent().map(|p| p.to_path_buf()).unwrap_or_default()).unwrap_or_default())
        .unwrap_or_else(|_| std::env::current_dir().unwrap_or_default());

    let csv_path = workspace_root.join("simulator/PlayerStatisticsExtended.csv");
    let log_path = workspace_root.join("axum_nba_strike.log");

    println!("Starting Axum Isolation Strike: NBA Dataset (Kernel-Space Ingestion)...");

    // --- PHASE 1: INGESTION ---
    let start_ingest = Instant::now();

    let src_file = File::open(&csv_path)?;
    let src_fd = src_file.as_raw_fd();
    let metadata = src_file.metadata()?;
    let file_size = metadata.len() as usize;

    let mut pipe = MemoryPipe::new(file_size)?;
    let dst_fd = pipe.get_fd();

    let mut ingestion_strategy = "Kernel-Space Splice (copy_file_range)";

    // Attempt Kernel-Space Splice
    let mut off_in: libc::loff_t = 0;
    let mut off_out: libc::loff_t = 0;

    let ret =
        unsafe { libc::copy_file_range(src_fd, &mut off_in, dst_fd, &mut off_out, file_size, 0) };

    if ret < 0 {
        let err = std::io::Error::last_os_error();
        if err.raw_os_error() == Some(18) || err.raw_os_error() == Some(libc::EXDEV) {
            println!("copy_file_range failed (EXDEV), falling back to Mmap Zero-Copy Ingestion...");
            ingestion_strategy = "Mmap Zero-Copy (Fallback from EXDEV)";

            // Fallback: mmap source, write to pipe
            let mmap = unsafe { memmap2::Mmap::map(&src_file)? };
            pipe.write_and_seal(&mmap)?;
        } else {
            return Err(format!("Kernel Splice Failure: {}", err).into());
        }
    } else {
        let mut total_copied: i64 = ret as i64;
        while total_copied < file_size as i64 {
            let bytes_to_copy = std::cmp::min(1024 * 1024 * 1024, file_size as i64 - total_copied);
            let ret = unsafe {
                libc::copy_file_range(
                    src_fd,
                    &mut off_in,
                    dst_fd,
                    &mut off_out,
                    bytes_to_copy as usize,
                    0,
                )
            };
            if ret <= 0 {
                break;
            }
            total_copied += ret as i64;
        }
        pipe.seal()?;
    }

    let dup_fd = unsafe { libc::dup(dst_fd) };
    if dup_fd < 0 {
        return Err("Failed to duplicate FD".into());
    }
    let owned_fd = unsafe { OwnedFd::from_raw_fd(dup_fd) };

    let ingest_latency = start_ingest.elapsed();

    // Calculate column count (minimal read)
    let mut header_buf = [0u8; 4096];
    let mut h_file = File::open(&csv_path)?;
    use std::io::Read;
    let n = h_file.read(&mut header_buf)?;
    let header_str = String::from_utf8_lossy(&header_buf[..n]);
    let first_line = header_str.lines().next().unwrap_or("");
    let column_count = first_line.split(',').count();

    // --- PHASE 2: ANALYSIS ---
    let start_analyze = Instant::now();
    let engine = AnalyzeEngine::new()?;
    let analytics = engine.analyze(owned_fd, "points")?;
    let analyze_latency = start_analyze.elapsed();
    let total_latency = start_total.elapsed();

    // Memory check
    let peak_rss = get_peak_rss();

    // --- PHASE 3: FORENSIC LOGGING ---
    let mut log_file = File::create(&log_path)?;
    let mut report = String::new();
    report.push_str("=== FORENSIC AUDIT REPORT: AXUM ISOLATION STRIKE ===\n\n");
    report.push_str("[METADATA]\n");
    report.push_str(&format!("Source File: {}\n", csv_path.display()));
    report.push_str(&format!("Total Rows: {}\n", analytics.total_rows));
    report.push_str(&format!("Total Columns: {}\n\n", column_count));

    report.push_str("[PHYSICS]\n");
    report.push_str(&format!("Ingestion Strategy: {}\n", ingestion_strategy));
    report.push_str(&format!(
        "Peak Memory: {:.2} MB\n\n",
        peak_rss as f64 / (1024.0 * 1024.0)
    ));

    report.push_str("[LATENCY]\n");
    report.push_str(&format!(
        "Ingestion Time: {} ms\n",
        ingest_latency.as_millis()
    ));
    report.push_str(&format!(
        "Analysis Time: {} ms\n",
        analyze_latency.as_millis()
    ));
    report.push_str(&format!(
        "Total Strike Time: {} ms\n\n",
        total_latency.as_millis()
    ));

    report.push_str("[AXIOM CRUNCH: PTS]\n");
    report.push_str(&format!("Mean: {:.4}\n", analytics.mean));
    report.push_str(&format!("Max: {:.4}\n", analytics.max));
    report.push_str(&format!("Std Dev: {:.4}\n", analytics.std_dev));
    report.push_str(&format!("P99: {:.4}\n", analytics.p99));
    report.push_str(&format!("Skewness: {:.4}\n", analytics.skewness));
    report.push_str(&format!("Kurtosis: {:.4}\n", analytics.kurtosis));
    report.push_str("\n");

    report.push_str("[ANOMALIES]\n");
    for (i, anomaly) in analytics.anomalies.iter().enumerate() {
        report.push_str(&format!("Anomaly #{}: {}\n", i + 1, anomaly));
    }

    log_file.write_all(report.as_bytes())?;
    println!("Strike complete. Strategy: {}", ingestion_strategy);
    println!("Total latency: {} ms", total_latency.as_millis());
    println!("\n=== SUMMARY ===");
    println!("Source File: {}", csv_path.display());
    println!("Total Rows: {}", analytics.total_rows);
    println!("Total Columns: {}", column_count);
    println!("Mean Points: {:.4}", analytics.mean);
    println!("Max Points: {:.4}", analytics.max);
    println!("Std Dev: {:.4}", analytics.std_dev);
    println!("P99: {:.4}", analytics.p99);
    println!("\nFull report saved to: {}\n", log_path.display());

    Ok(())
}
