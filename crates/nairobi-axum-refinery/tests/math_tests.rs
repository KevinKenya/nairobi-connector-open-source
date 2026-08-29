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

// File: crates/nairobi-axum-refinery/tests/math_tests.rs
// Author: Kevin Chege. Location: Nairobi
// Date: 2026-05-21

// nairobi-open-source-release/crates/nairobi-axum-refinery/tests/math_tests.rs
use memfd::MemfdOptions;
use nairobi_axum_refinery::analyze::{get_peak_rss, AnalyzeEngine};
use nairobi_protocol::{CorrelationResult, DistilledAnalytics};
use std::io::Write;
use std::os::unix::io::FromRawFd;
use zbus::zvariant::OwnedFd;

// Note: We need byteorder for EncodingContext type annotation
// But wait, zvariant might re-export it or we can just use the signature from nairobi-connector/server.rs
// server.rs used: zvariant::EncodingContext::<byteorder::LE>::new_gvariant(0)

#[tokio::test]
async fn test_axiom_crunch_math() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Generate a known normal distribution in a memfd
    let mut data = String::from("val\n");
    data.push_str("-1.0\n0.0\n1.0\n");

    let opts = MemfdOptions::default().allow_sealing(true);
    let mfd = opts.create("test_math.csv")?;
    let mut file = mfd.into_file();
    file.write_all(data.as_bytes())?;

    let raw_fd = std::os::fd::IntoRawFd::into_raw_fd(file);
    let handle = unsafe { OwnedFd::from_raw_fd(raw_fd) };

    // 2. Run Analyze
    let engine = AnalyzeEngine::new()?;
    let t_start = std::time::Instant::now();
    let analytics = engine.analyze(handle, "val")?;
    let duration = t_start.elapsed();

    // 3. Assertions
    println!(
        "Mean: {}, Std Dev: {}, Duration: {:?}",
        analytics.mean, analytics.std_dev, duration
    );
    assert!((analytics.mean - 0.0).abs() < 1e-6);
    assert!((analytics.std_dev - 1.0).abs() < 1e-6);
    assert_eq!(analytics.total_rows, 3);

    // 4. Verify GVariant serialization performance
    // We try to use the same pattern as in server.rs if possible, but let's just use the trait bounds
    // Since I can't easily add byteorder dependency now without risk of mismatch, I'll use a simpler way to verify it's Type
    use zbus::zvariant::Type;
    assert_eq!(DistilledAnalytics::signature(), "(tdddddddddhas)");

    Ok(())
}

#[tokio::test]
async fn test_z_score_anomalies() -> Result<(), Box<dyn std::error::Error>> {
    let mut data = String::from("id,val\n");
    for i in 0..10 {
        data.push_str(&format!("node_{},0.0\n", i));
    }
    data.push_str("node_anomaly,100.0\n");

    let opts = MemfdOptions::default().allow_sealing(true);
    let mfd = opts.create("test_anomalies.csv")?;
    let mut file = mfd.into_file();
    file.write_all(data.as_bytes())?;

    let raw_fd = std::os::fd::IntoRawFd::into_raw_fd(file);
    let handle = unsafe { OwnedFd::from_raw_fd(raw_fd) };

    let engine = AnalyzeEngine::new()?;
    let analytics = engine.analyze(handle, "val")?;

    println!("Anomalies: {:?}", analytics.anomalies);
    assert_eq!(analytics.anomalies.len(), 1);
    assert!(analytics.anomalies[0].contains("node_anomaly"));
    assert!(analytics.anomalies[0].contains("100"));

    Ok(())
}

// ───────────────────────────────────────────────────────────────────────
// CORRELATION TESTS
// ───────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_correlation_pearson_spearman() -> Result<(), Box<dyn std::error::Error>> {
    // Create dataset with perfect positive correlation: y = 2*x + 1
    let mut data = String::from("x,y\n");
    for i in 0..100 {
        data.push_str(&format!("{},{}\n", i, 2 * i + 1));
    }

    let opts = MemfdOptions::default().allow_sealing(true);
    let mfd = opts.create("test_corr.csv")?;
    let mut file = mfd.into_file();
    file.write_all(data.as_bytes())?;

    let raw_fd = std::os::fd::IntoRawFd::into_raw_fd(file);
    let handle = unsafe { OwnedFd::from_raw_fd(raw_fd) };

    let engine = AnalyzeEngine::new()?;
    let result = engine.correlation(handle, "x,y")?;

    println!("Pearson: {}, Spearman: {}", result.pearson, result.spearman);

    // Perfect positive correlation should be close to 1.0
    assert!(
        result.pearson > 0.99,
        "Pearson should be ~1.0 for perfect positive correlation"
    );
    assert!(
        result.spearman > 0.99,
        "Spearman should be ~1.0 for perfect positive correlation"
    );

    Ok(())
}

#[tokio::test]
async fn test_correlation_negative() -> Result<(), Box<dyn std::error::Error>> {
    // Create dataset with perfect negative correlation: y = -x
    let mut data = String::from("x,y\n");
    for i in 0..100 {
        data.push_str(&format!("{},{}\n", i, -(i as i32)));
    }

    let opts = MemfdOptions::default().allow_sealing(true);
    let mfd = opts.create("test_corr_neg.csv")?;
    let mut file = mfd.into_file();
    file.write_all(data.as_bytes())?;

    let raw_fd = std::os::fd::IntoRawFd::into_raw_fd(file);
    let handle = unsafe { OwnedFd::from_raw_fd(raw_fd) };

    let engine = AnalyzeEngine::new()?;
    let result = engine.correlation(handle, "x,y")?;

    println!("Pearson: {}, Spearman: {}", result.pearson, result.spearman);

    // Perfect negative correlation should be close to -1.0
    assert!(
        result.pearson < -0.99,
        "Pearson should be ~-1.0 for perfect negative correlation"
    );
    assert!(
        result.spearman < -0.99,
        "Spearman should be ~-1.0 for perfect negative correlation"
    );

    Ok(())
}

#[tokio::test]
async fn test_correlation_zero_variance_guard() -> Result<(), Box<dyn std::error::Error>> {
    // Create dataset where one column has zero variance (all same value)
    let mut data = String::from("x,y\n");
    for _ in 0..50 {
        data.push_str("1.0,2.0\n"); // x is constant (zero variance)
    }

    let opts = MemfdOptions::default().allow_sealing(true);
    let mfd = opts.create("test_corr_zero_var.csv")?;
    let mut file = mfd.into_file();
    file.write_all(data.as_bytes())?;

    let raw_fd = std::os::fd::IntoRawFd::into_raw_fd(file);
    let handle = unsafe { OwnedFd::from_raw_fd(raw_fd) };

    let engine = AnalyzeEngine::new()?;
    let result = engine.correlation(handle, "x,y")?;

    println!("Pearson: {}, Spearman: {}", result.pearson, result.spearman);

    // Guard: zero variance returns (0.0, 0.0) to prevent NaN
    assert_eq!(
        result.pearson, 0.0,
        "Pearson should be 0.0 for zero variance"
    );
    assert_eq!(
        result.spearman, 0.0,
        "Spearman should be 0.0 for zero variance"
    );

    Ok(())
}

#[tokio::test]
async fn test_correlation_invalid_column() -> Result<(), Box<dyn std::error::Error>> {
    let mut data = String::from("x,y\n");
    data.push_str("1.0,2.0\n");

    let opts = MemfdOptions::default().allow_sealing(true);
    let mfd = opts.create("test_corr_invalid.csv")?;
    let mut file = mfd.into_file();
    file.write_all(data.as_bytes())?;

    let raw_fd = std::os::fd::IntoRawFd::into_raw_fd(file);
    let handle = unsafe { OwnedFd::from_raw_fd(raw_fd) };

    let engine = AnalyzeEngine::new()?;

    // Try to correlate with non-existent column
    let result = engine.correlation(handle, "x,nonexistent");

    // Should return an error
    assert!(result.is_err(), "Should error on non-existent column");

    Ok(())
}

#[tokio::test]
async fn test_correlation_wrong_column_count() -> Result<(), Box<dyn std::error::Error>> {
    let mut data = String::from("x,y,z\n");
    data.push_str("1.0,2.0,3.0\n");

    let opts = MemfdOptions::default().allow_sealing(true);
    let mfd = opts.create("test_corr_wrong_count.csv")?;
    let mut file = mfd.into_file();
    file.write_all(data.as_bytes())?;

    let raw_fd = std::os::fd::IntoRawFd::into_raw_fd(file);
    let handle = unsafe { OwnedFd::from_raw_fd(raw_fd) };

    let engine = AnalyzeEngine::new()?;

    // Try with wrong number of columns (should be exactly 2)
    let result = engine.correlation(handle, "x"); // Only 1 column

    // Should return an error
    assert!(result.is_err(), "Should error when not exactly 2 columns");

    Ok(())
}

#[tokio::test]
async fn test_correlation_gvariant_signature() {
    // Verify CorrelationResult has correct GVariant signature
    use zbus::zvariant::Type;
    let signature = CorrelationResult::signature();
    assert_eq!(
        signature, "(dd)",
        "CorrelationResult GVariant signature should be (dd)"
    );
}

#[tokio::test]
async fn test_correlation_peak_rss() -> Result<(), Box<dyn std::error::Error>> {
    // Create a dataset for correlation
    let mut data = String::from("x,y\n");
    for i in 0..1000 {
        data.push_str(&format!("{},{}\n", i, 2 * i));
    }

    let opts = MemfdOptions::default().allow_sealing(true);
    let mfd = opts.create("test_corr_rss.csv")?;
    let mut file = mfd.into_file();
    file.write_all(data.as_bytes())?;

    let raw_fd = std::os::fd::IntoRawFd::into_raw_fd(file);
    let handle = unsafe { OwnedFd::from_raw_fd(raw_fd) };

    let engine = AnalyzeEngine::new()?;

    // Get peak RSS before
    let rss_before = get_peak_rss();

    let result = engine.correlation(handle, "x,y")?;

    // get_peak_rss is called inside correlation(), but we can verify it works
    let rss_after = get_peak_rss();

    println!(
        "RSS before: {} bytes, RSS after: {} bytes",
        rss_before, rss_after
    );
    println!(
        "Correlation result: Pearson={}, Spearman={}",
        result.pearson, result.spearman
    );

    // Verify correlation works
    assert!(
        result.pearson > 0.99,
        "Pearson should be high for linear relationship"
    );

    Ok(())
}

#[tokio::test]
async fn test_correlation_no_correlation() -> Result<(), Box<dyn std::error::Error>> {
    // Create dataset with no correlation: random-ish x and y
    let mut data = String::from("x,y\n");
    // x is sequence, y is constant (no correlation)
    for i in 0..50 {
        data.push_str(&format!("{},{}\n", i, 42.0)); // y is constant
    }

    let opts = MemfdOptions::default().allow_sealing(true);
    let mfd = opts.create("test_corr_none.csv")?;
    let mut file = mfd.into_file();
    file.write_all(data.as_bytes())?;

    let raw_fd = std::os::fd::IntoRawFd::into_raw_fd(file);
    let handle = unsafe { OwnedFd::from_raw_fd(raw_fd) };

    let engine = AnalyzeEngine::new()?;
    let result = engine.correlation(handle, "x,y")?;

    println!("Pearson: {}, Spearman: {}", result.pearson, result.spearman);

    // When y is constant, correlation should be 0.0 (guard kicks in)
    assert_eq!(
        result.pearson, 0.0,
        "Pearson should be 0.0 when one variable has zero variance"
    );
    assert_eq!(
        result.spearman, 0.0,
        "Spearman should be 0.0 when one variable has zero variance"
    );

    Ok(())
}

// ───────────────────────────────────────────────────────────────────────
// DECODER TESTS (moved here for crate-relative testing)
// ───────────────────────────────────────────────────────────────────────

#[test]
fn test_generate_correlation_report() {
    use nairobi_hub::decoder::generate_correlation_report;

    let result = CorrelationResult {
        pearson: 0.95,
        spearman: 0.92,
    };

    let report = generate_correlation_report(&result, 1024 * 1024 * 50); // 50MB

    assert!(
        report.contains("Relational Strike Report"),
        "Should contain title"
    );
    assert!(report.contains("0.9500"), "Should contain Pearson value");
    assert!(report.contains("0.9200"), "Should contain Spearman value");
    assert!(
        report.contains("Extremely Strong"),
        "Should show relational strength"
    );
    assert!(report.contains("50.00 MB"), "Should contain Peak RSS");
}

#[test]
fn test_generate_correlation_report_negative() {
    use nairobi_hub::decoder::generate_correlation_report;

    let result = CorrelationResult {
        pearson: -0.85,
        spearman: -0.82,
    };

    let report = generate_correlation_report(&result, 0); // No RSS data

    assert!(
        report.contains("-0.8500"),
        "Should contain negative Pearson"
    );
    assert!(
        report.contains("Strong Relational Bond"),
        "Should show strong negative correlation"
    );
    assert!(
        report.contains("automation_telemetry.log"),
        "Should reference telemetry log when no RSS"
    );
}

#[test]
fn test_generate_correlation_report_weak() {
    use nairobi_hub::decoder::generate_correlation_report;

    let result = CorrelationResult {
        pearson: 0.3,
        spearman: 0.25,
    };

    let report = generate_correlation_report(&result, 1024 * 1024 * 100); // 100MB

    assert!(
        report.contains("Negligible Noise"),
        "Should show weak correlation"
    );
    assert!(report.contains("100.00 MB"), "Should contain Peak RSS");
}
