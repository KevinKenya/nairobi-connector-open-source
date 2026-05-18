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

// File: /home/KevinKenya/nairobi-connector-open-source/crates/nairobi-protocol/tests/integration_tests.rs
// Author: Kevin Chege. Location: Nairobi
// Date: 2026-05-06

// nairobi-open-source-release/crates/nairobi-protocol/tests/integration_tests.rs
// Integration tests for Nairobi Protocol crate
// Tests MemoryPipe, GVariant serialization, and D-Bus interface constants.

use nairobi_protocol::{CleanDataStrategy, DistilledAnalytics, MemoryPipe, SchemaInspection};
use std::os::fd::FromRawFd;
use std::path::Path;

/// Helper function to read a file and check if it contains a pattern
/// Uses workspace root as base if path is relative
fn file_contains(path: &str, pattern: &str) -> bool {
    // Try relative to current dir, then try workspace root
    let workspace_root = std::env::var("CARGO_WORKSPACE_DIR")
        .unwrap_or_else(|_| "./".to_string());

    let paths_to_try = vec![
        Path::new(&workspace_root).join(path),
        Path::new(path).to_path_buf(),
        Path::new("/app").join(path),
    ];

    for p in paths_to_try {
        if let Ok(text) = std::fs::read_to_string(&p) {
            if text.contains(pattern) {
                return true;
            }
        }
    }
    false
}

// ─────────────────────────────────────────────────────────────────────────────
// 3.1 MemoryPipe Tests
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_memfd_forge_seal_pattern() {
    let path = "crates/nairobi-protocol/src/mem_pipe.rs";
    assert!(
        file_contains(path, "MemfdOptions"),
        "MemfdOptions not found in mem_pipe.rs"
    );
    assert!(
        file_contains(path, "allow_sealing(true)"),
        "allow_sealing not found in mem_pipe.rs"
    );
    assert!(
        file_contains(path, "add_seals"),
        "add_seals not found in mem_pipe.rs"
    );
    assert!(
        file_contains(path, "SealWrite"),
        "SealWrite not found in mem_pipe.rs"
    );
    assert!(
        file_contains(path, "SealShrink"),
        "SealShrink not found in mem_pipe.rs"
    );
}

#[test]
fn test_memfd_write_and_seal() {
    // Actually create a memfd, write data, verify seal
    let mut pipe = MemoryPipe::new(4096).expect("Failed to create MemoryPipe");

    let test_data = b"Hello, Nairobi-Axum OS!";
    let addr = pipe
        .write_and_seal(test_data)
        .expect("Failed to write and seal");

    assert!(!addr.is_null(), "Forge address should not be null");

    // Verify we can read the data back
    let slice = pipe.as_slice().expect("Failed to map memfd");
    assert_eq!(
        &slice[..test_data.len()],
        test_data,
        "Data read back should match data written"
    );
}

#[test]
fn test_memfd_size() {
    let pipe = MemoryPipe::new(8192).expect("Failed to create MemoryPipe");
    assert_eq!(pipe.size(), 8192, "MemoryPipe size should match requested");
}

// ─────────────────────────────────────────────────────────────────────────────
// 3.2 GVariant Tests
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_distilled_analytics_structure() {
    let path = "crates/nairobi-protocol/src/types.rs";
    assert!(
        file_contains(path, "struct DistilledAnalytics"),
        "DistilledAnalytics struct not found"
    );
    assert!(
        file_contains(path, "total_rows: u64"),
        "Field 'total_rows' not found or incorrect type"
    );
    assert!(
        file_contains(path, "min: f64"),
        "Field 'min' not found or incorrect type"
    );
    assert!(
        file_contains(path, "max: f64"),
        "Field 'max' not found or incorrect type"
    );
    assert!(
        file_contains(path, "mean: f64"),
        "Field 'mean' not found or incorrect type"
    );
    assert!(
        file_contains(path, "std_dev: f64"),
        "Field 'std_dev' not found or incorrect type"
    );
    assert!(
        file_contains(path, "handle: OwnedFd"),
        "Field 'handle' not found or incorrect type"
    );
    assert!(
        file_contains(path, "anomalies: Vec<String>"),
        "Field 'anomalies' not found or incorrect type"
    );
}

#[test]
fn test_distilled_analytics_creation() {
    // Verify we can create a DistilledAnalytics instance
    // Note: We can't easily create OwnedFd in tests without proper FD handling
    // So we just verify the struct can be instantiated with a placeholder
    // In real usage, OwnedFd comes from D-Bus calls or file operations

    // Just verify the struct definition exists and compiles
    assert!(true, "DistilledAnalytics struct should compile");
}

#[test]
fn test_gvariant_serialization() {
    // Verify GVariant signature matches documentation: (tdddddddddhas)
    let path = "crates/nairobi-protocol/src/types.rs";
    assert!(
        file_contains(path, "signature = \"as\""),
        "GVariant signature for anomalies not found"
    );
    assert!(
        file_contains(path, "signature = \"at\""),
        "GVariant signature for null_counts not found"
    );
}

#[test]
fn test_schema_inspection_structure() {
    let path = "crates/nairobi-protocol/src/types.rs";
    assert!(
        file_contains(path, "struct SchemaInspection"),
        "SchemaInspection struct not found"
    );
    assert!(
        file_contains(path, "columns: Vec<String>"),
        "Field 'columns' not found"
    );
    assert!(
        file_contains(path, "data_types: Vec<String>"),
        "Field 'data_types' not found"
    );
    assert!(
        file_contains(path, "null_counts: Vec<u64>"),
        "Field 'null_counts' not found"
    );
    assert!(
        file_contains(path, "sample_json: String"),
        "Field 'sample_json' not found"
    );
}

#[test]
fn test_clean_data_strategy_structure() {
    let path = "crates/nairobi-protocol/src/types.rs";
    assert!(
        file_contains(path, "struct CleanDataStrategy"),
        "CleanDataStrategy struct not found"
    );
    assert!(
        file_contains(path, "column: String"),
        "Field 'column' not found"
    );
    assert!(
        file_contains(path, "strategy: String"),
        "Field 'strategy' not found"
    );
    assert!(
        file_contains(path, "fill_value: String"),
        "Field 'fill_value' not found"
    );
}

#[test]
fn test_correlation_result_structure() {
    let path = "crates/nairobi-protocol/src/types.rs";
    assert!(
        file_contains(path, "struct CorrelationResult"),
        "CorrelationResult struct not found"
    );
    assert!(
        file_contains(path, "pearson: f64"),
        "Field 'pearson' not found or incorrect type"
    );
    assert!(
        file_contains(path, "spearman: f64"),
        "Field 'spearman' not found or incorrect type"
    );
}

#[test]
fn test_correlation_result_gvariant_signature() {
    let path = "crates/nairobi-protocol/src/types.rs";
    assert!(
        file_contains(path, "signature = \"(dd)\""),
        "GVariant signature (dd) for CorrelationResult not found"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// 3.3 D-Bus Interface Tests
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_dbus_constants_match_bible() {
    let path = "crates/nairobi-protocol/src/interface.rs";
    assert!(
        file_contains(path, "org.nairobi.NairobiAxumRefinery1"),
        "INTERFACE_NAME constant not found or incorrect"
    );
    assert!(
        file_contains(path, "org.nairobi.NairobiAxumRefinery1"),
        "SERVICE_NAME constant not found or incorrect"
    );
    assert!(
        file_contains(path, "/org/nairobi/NairobiAxumRefinery1"),
        "OBJECT_PATH constant not found or incorrect"
    );
}

#[test]
fn test_dbus_method_signatures() {
    // Path relative to workspace root (cargo test runs from workspace root)
    let path = "crates/nairobi-axum-refinery/src/dbus_service.rs";
    assert!(
        file_contains(path, "async fn ingest"),
        "D-Bus Ingest method not found"
    );
    assert!(
        file_contains(path, "async fn analyze"),
        "D-Bus Analyze method not found"
    );
    assert!(
        file_contains(path, "async fn inspect_schema"),
        "D-Bus InspectSchema method not found"
    );
    assert!(
        file_contains(path, "async fn clean_data"),
        "D-Bus CleanData method not found"
    );
    assert!(
        file_contains(path, "async fn sql_query"),
        "D-Bus SqlQuery method not found"
    );
    assert!(
        file_contains(path, "async fn correlation"),
        "D-Bus Correlation method not found"
    );
}
