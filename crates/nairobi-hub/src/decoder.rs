// File: /home/chege/nairobi-connector-open-source/crates/nairobi-hub/src/decoder.rs
// Author: Kevin Chege. Location: Nairobi
// Date: 2026-05-06

// nairobi-open-source-release/crates/nairobi-hub/src/decoder.rs
use nairobi_protocol::types::{CorrelationResult, DistilledAnalytics};

/// Translates the binary GVariant payload into a clean, professional Markdown report.
pub fn generate_report(analytics: &DistilledAnalytics) -> String {
    let mut report = String::new();

    report.push_str("# Nairobi Connector OS: Neural Handshake Report\n\n");
    report.push_str("## Analytics Summary\n");
    report.push_str(&format!("- **Total Rows:** {}\n", analytics.total_rows));
    report.push_str(&format!("- **Minimum Value:** {:.4}\n", analytics.min));
    report.push_str(&format!("- **Maximum Value:** {:.4}\n", analytics.max));
    report.push_str(&format!("- **Mean Value:** {:.4}\n\n", analytics.mean));

    report.push_str("## Distribution Profile\n");
    report.push_str("### Volatility\n");
    report.push_str(&format!(
        "- **Standard Deviation:** {:.4}\n",
        analytics.std_dev
    ));
    report.push_str(&format!("- **Variance:** {:.4}\n", analytics.variance));

    report.push_str("\n### Tail Risk\n");
    report.push_str(&format!(
        "- **P95 (95th Percentile):** {:.4}\n",
        analytics.p95
    ));
    report.push_str(&format!(
        "- **P99 (99th Percentile):** {:.4}\n",
        analytics.p99
    ));

    report.push_str("\n### Shape\n");
    report.push_str(&format!("- **Skewness:** {:.4}\n", analytics.skewness));
    report.push_str(&format!("- **Kurtosis:** {:.4}\n", analytics.kurtosis));

    if analytics.kurtosis > 3.0 {
        report.push_str(
            "  - *Semantic Hint:* High Kurtosis indicates frequent outliers and fat tails.\n",
        );
    } else if analytics.kurtosis < -1.0 {
        report.push_str("  - *Semantic Hint:* Low Kurtosis indicates a lack of outliers.\n");
    }

    if !analytics.anomalies.is_empty() {
        report.push_str("\n## Forensic Signal: Top Scorers / Anomalies\n");
        report.push_str("> Rows exceeding 3.0 Standard Deviations from the mean.\n\n");
        for anomaly in &analytics.anomalies {
            // If the anomaly represents a raw CSV row (e.g., contains a comma),
            // wrap it in a Markdown code block to ensure the LLM parses it as structured data.
            if anomaly.contains(',') {
                report.push_str("```csv\n");
                report.push_str(anomaly);
                report.push_str("\n```\n");
            } else {
                report.push_str(&format!("- {}\n", anomaly));
            }
        }
    }

    report
}

/// Translates correlation results into a "Relational Strength" report.
pub fn generate_correlation_report(result: &CorrelationResult, peak_rss: u64) -> String {
    let mut report = String::new();
    report.push_str("# Nairobi Connector OS: Relational Strike Report\n\n");

    report.push_str("## Correlation Matrix\n");
    report.push_str(&format!(
        "- **Pearson Coefficient:** {:.4}\n",
        result.pearson
    ));
    report.push_str(&format!(
        "- **Spearman Rank Coefficient:** {:.4}\n\n",
        result.spearman
    ));

    report.push_str("## Relational Strength\n");
    let strength = |val: f64| {
        let abs_val = val.abs();
        if abs_val >= 0.95 {
            if val > 0.0 {
                "Extremely Strong Positive Correlation"
            } else if val < 0.0 {
                "Extremely Strong Negative Correlation"
            } else {
                "Extremely Strong Correlation"
            }
        } else if abs_val >= 0.7 {
            "Strong Relational Bond"
        } else if abs_val >= 0.4 {
            "Moderate Signal"
        } else {
            "Negligible Noise"
        }
    };

    report.push_str(&format!(
        "- **Pearson Interpretation:** {}\n",
        strength(result.pearson)
    ));
    report.push_str(&format!(
        "- **Spearman Interpretation:** {}\n\n",
        strength(result.spearman)
    ));

    report.push_str("## Infrastructure Metrics\n");
    if peak_rss > 0 {
        report.push_str(&format!(
            "- **Compute Density Metric (Peak RSS):** {:.2} MB\n",
            peak_rss as f64 / 1024.0 / 1024.0
        ));
    } else {
        report.push_str("- **Compute Density Metric (Peak RSS):** See automation_telemetry.log [DENSITY_AUDIT]\n");
    }

    report
}
