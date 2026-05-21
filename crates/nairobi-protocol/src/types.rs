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

// File: /home/KevinKenya/nairobi-connector-open-source/crates/nairobi-protocol/src/types.rs
// Author: Kevin Chege. Location: Nairobi
// Date: 2026-05-21

// nairobi-open-source-release/crates/nairobi-protocol/src/types.rs
//! GVariant-compatible types for the D-Bus IPC protocol.
//!
//! These types are serialized/deserialized across process boundaries
//! using GVariant encoding. They constitute the "vocabulary" of the
//! Axum Refinery ↔ Nairobi Hub handshake.

use serde::{Deserialize, Serialize};
use zbus::zvariant::{OwnedFd, Type};

/// Distilled analytics result — the `v` payload returned by `Analyze`.
///
/// GVariant signature: `(tdddddddddhas)`
/// - `t` = total_rows (u64)
/// - `d` = min, max, mean, std_dev, variance, p95, p99, skewness, kurtosis (f64)
/// - `h` = handle (OwnedFd — the memfd back-reference)
/// - `as` = anomalies (Vec<String>)
///
/// # Process Isolation
/// Anomalies use `Vec<String>` (owned) instead of `&str` because
/// borrowed data cannot cross the D-Bus process boundary. The zero-copy
/// memfd still handles the 1GB payload; anomalies are the distilled signal.
#[derive(Debug, Serialize, Deserialize, Type)]
pub struct DistilledAnalytics {
    pub total_rows: u64,
    pub min: f64,
    pub max: f64,
    pub mean: f64,
    pub std_dev: f64,
    pub variance: f64,
    pub p95: f64,
    pub p99: f64,
    pub skewness: f64,
    pub kurtosis: f64,
    pub handle: OwnedFd,
    #[zvariant(signature = "as")]
    pub anomalies: Vec<String>,
}

/// Result of the `InspectSchema` call.
#[derive(Debug, Serialize, Deserialize, Type)]
pub struct SchemaInspection {
    #[zvariant(signature = "as")]
    pub columns: Vec<String>,
    #[zvariant(signature = "as")]
    pub data_types: Vec<String>,
    #[zvariant(signature = "at")]
    pub null_counts: Vec<u64>,
    pub sample_json: String,
    pub handle: OwnedFd,
}

/// Strategy for cleaning a specific column.
#[derive(Debug, Serialize, Deserialize, Type)]
pub struct CleanDataStrategy {
    pub column: String,
    pub strategy: String,   // e.g., "fill_zero", "drop_row", "drop_column"
    pub fill_value: String, // Used if strategy is "fill_value"
}

/// Result of the `Correlation` call.
///
/// GVariant signature: `(dd)`
/// - Pearson correlation coefficient
/// - Spearman rank correlation coefficient
#[derive(Debug, Serialize, Deserialize, Type)]
#[zvariant(signature = "(dd)")]
pub struct CorrelationResult {
    pub pearson: f64,
    pub spearman: f64,
}

/// Fused analytics + correlation result — returned by the combined
/// `CrunchAndCorrelate` and `IngestCrunchCorrelate` D-Bus methods.
///
/// This eliminates multiple D-Bus round trips and avoids re-parsing
/// the CSV from memfd. All analytics and correlation are computed from
/// a single DataFrame parse.
///
/// GVariant signature: `(tdddddddddddasdd)`
#[derive(Debug, Serialize, Deserialize, Type)]
pub struct FusedAnalyticsResult {
    // ── Analytics fields ──
    pub total_rows: u64,
    pub min: f64,
    pub max: f64,
    pub mean: f64,
    pub std_dev: f64,
    pub variance: f64,
    pub p95: f64,
    pub p99: f64,
    pub skewness: f64,
    pub kurtosis: f64,
    // ── Correlation fields ──
    pub pearson: f64,
    pub spearman: f64,
    // ── Anomalies ──
    #[zvariant(signature = "as")]
    pub anomalies: Vec<String>,
}
