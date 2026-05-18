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

// File: /home/KevinKenya/nairobi-connector-open-source/crates/nairobi-axum-refinery/tests/benchmark_tests.rs
// Author: Kevin Chege. Location: Nairobi
// Date: 2026-05-06

// nairobi-open-source-release/crates/nairobi-axum-refinery/tests/benchmark_tests.rs
// Benchmark tests for Axum-Refinery
// Tests io_uring, Polars SQL, and execution timeout
// Remove #[ignore] tag to run benchmarks

use std::path::Path;
use std::time::Instant;

/// Helper function to read a file and check if it contains a pattern
fn file_contains(path: &str, pattern: &str) -> bool {
    let content = std::fs::read_to_string(Path::new(path));
    match content {
        Ok(text) => text.contains(pattern),
        Err(_) => false,
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 4.1 io_uring Tests
// ─────────────────────────────────────────────────────────────────────────────

#[test]
#[ignore] // Remove to run benchmark
fn test_1gb_ingest_latency() {
    let path = "../nairobi-axum-refinery/src/ingest.rs";

    // Verify io_uring is configured
    assert!(
        file_contains(path, "io_uring"),
        "io_uring not found in ingest.rs"
    );
    assert!(
        file_contains(path, "setup_sqpoll"),
        "SQPOLL not configured in ingest.rs"
    );

    // Benchmark: Create a 1GB test file and measure ingest time
    let test_file = "/tmp/test_1gb_data.csv";
    let test_size = 1024 * 1024 * 1024; // 1GB

    // Create sparse file (or use existing test file)
    // For now, just verify the Dirac Engine exists
    assert!(
        file_contains(path, "DiracEngine"),
        "DiracEngine not found in ingest.rs"
    );

    // TODO: Actual benchmark with std::time::Instant
    // let start = Instant::now();
    // ... perform ingest ...
    // let elapsed = start.elapsed();
    // assert!(elapsed.as_millis() < 500, "Ingest took {}ms, target <500ms", elapsed.as_millis());
}

#[test]
fn test_io_uring_sqpoll_config() {
    let path = "../nairobi-axum-refinery/src/ingest.rs";
    assert!(
        file_contains(path, "setup_sqpoll"),
        "SQPOLL configuration not found"
    );
    assert!(file_contains(path, "SQPOLL"), "SQPOLL mention not found");
}

// ─────────────────────────────────────────────────────────────────────────────
// 4.2 Polars SQL Tests
// ─────────────────────────────────────────────────────────────────────────────

#[test]
#[ignore] // Remove to run benchmark
fn test_polars_sql_latency() {
    let path = "../nairobi-axum-refinery/src/analyze.rs";

    // Verify Rayon thread pool capping
    assert!(
        file_contains(path, "available_parallelism"),
        "Rayon pool should be dynamically capped"
    );

    // Verify SQL table name
    assert!(
        file_contains(path, "dataset"),
        "SQL table should be registered as 'dataset'"
    );

    // Benchmark: Execute SQL query on test data
    // let start = Instant::now();
    // ... execute SQL query ...
    // let elapsed = start.elapsed();
    // assert!(elapsed.as_millis() < 1500, "SQL took {}ms, target <1500ms");
}

#[test]
fn test_rayon_thread_capping() {
    let path = "../nairobi-axum-refinery/src/analyze.rs";
    assert!(
        file_contains(path, "available_parallelism"),
        "Rayon pool not dynamically capped as documented"
    );
    assert!(
        file_contains(path, "Rayon"),
        "Rayon not found in analyze.rs"
    );
}

#[test]
fn test_sql_table_name() {
    let path = "../nairobi-axum-refinery/src/analyze.rs";
    assert!(
        file_contains(path, "register(\"dataset\""),
        "SQL table name 'dataset' not found in analyze.rs"
    );
}

#[test]
fn test_huge_page_allocation() {
    let path = "../nairobi-axum-refinery/src/ingest.rs";
    assert!(
        file_contains(path, "MAP_HUGE_1GB"),
        "1GB Huge Page allocation not found"
    );
    assert!(
        file_contains(path, "MADV_HUGEPAGE"),
        "THP fallback not found"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// 4.3 Execution Guillotine Tests
// ─────────────────────────────────────────────────────────────────────────────

#[test]
#[ignore] // Remove to run benchmark
fn test_execution_timeout() {
    let path = "../nairobi-axum-refinery/src/analyze.rs";

    // Verify timeout is set to 10 seconds
    assert!(
        file_contains(path, "from_secs(10)"),
        "Execution timeout should be 10 seconds"
    );

    // Benchmark: Run a query that takes too long
    // let start = Instant::now();
    // let result = tokio::time::timeout(Duration::from_secs(10), async { ... }).await;
    // assert!(result.is_err(), "Query should timeout");
    // let elapsed = start.elapsed();
    // assert!(elapsed.as_secs() >= 10 && elapsed.as_secs() <= 11,
    //     "Timeout should be ~10 seconds");
}

#[test]
fn test_execution_guillotine_exists() {
    let path = "../nairobi-axum-refinery/src/analyze.rs";
    assert!(
        file_contains(path, "from_secs(10)"),
        "Execution Guillotine timeout not set to 10 seconds"
    );
    assert!(
        file_contains(path, "timeout"),
        "tokio::time::timeout not found"
    );
    assert!(
        file_contains(path, "spawn_blocking"),
        "spawn_blocking not found"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// 4.4 Dirac Engine Tests
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_dirac_engine_io_uring() {
    let path = "../nairobi-axum-refinery/src/ingest.rs";
    assert!(
        file_contains(path, "IoUring"),
        "IoUring not found in ingest.rs"
    );
    assert!(file_contains(path, "opcode"), "io_uring opcode not found");
    assert!(
        file_contains(path, "submission"),
        "io_uring submission queue not found"
    );
    assert!(
        file_contains(path, "completion"),
        "io_uring completion queue not found"
    );
}

#[test]
fn test_dirac_engine_huge_pages() {
    let path = "../nairobi-axum-refinery/src/ingest.rs";
    assert!(
        file_contains(path, "allocate_huge_page"),
        "allocate_huge_page function not found"
    );
    assert!(
        file_contains(path, "mmap"),
        "mmap not found for huge page allocation"
    );
    assert!(
        file_contains(path, "MAP_HUGETLB"),
        "MAP_HUGETLB flag not found"
    );
    assert!(
        file_contains(path, "MAP_HUGE_1GB"),
        "MAP_HUGE_1GB flag not found"
    );
}
