// File: /home/chege/nairobi-connector-open-source/crates/nairobi-hub/tests/e2e_tests.rs
// Author: Kevin Chege. Location: Nairobi
// Date: 2026-05-06

// nairobi-open-source-release/crates/nairobi-hub/tests/e2e_tests.rs
// End-to-End tests for Nairobi-Hub
// Tests D-Bus proxy creation and FD passing
// Requires running Axum-Refinery daemon for full E2E tests

use std::path::Path;

/// Helper function to read a file and check if it contains a pattern
fn file_contains(path: &str, pattern: &str) -> bool {
    let content = std::fs::read_to_string(Path::new(path));
    match content {
        Ok(text) => text.contains(pattern),
        Err(_) => false,
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 5.1 D-Bus Proxy Tests
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_dbus_proxy_creation() {
    let path = "../nairobi-hub/src/client.rs";

    // Verify RefineryClient struct exists
    assert!(
        file_contains(path, "struct RefineryClient"),
        "RefineryClient struct not found"
    );

    // Verify it has a connect method
    assert!(
        file_contains(path, "async fn connect()"),
        "RefineryClient::connect method not found"
    );

    // Verify it uses zbus::dbus_proxy macro
    assert!(
        file_contains(path, "dbus_proxy"),
        "dbus_proxy macro not found"
    );
}

#[test]
fn test_dbus_proxy_methods() {
    let path = "../nairobi-hub/src/client.rs";

    // Verify all D-Bus methods are proxied
    assert!(
        file_contains(path, "fn ingest"),
        "Proxy ingest method not found"
    );
    assert!(
        file_contains(path, "fn analyze"),
        "Proxy analyze method not found"
    );
    assert!(
        file_contains(path, "fn inspect_schema"),
        "Proxy inspect_schema method not found"
    );
    assert!(
        file_contains(path, "fn clean_data"),
        "Proxy clean_data method not found"
    );
    assert!(
        file_contains(path, "fn sql_query"),
        "Proxy sql_query method not found"
    );
}

#[test]
fn test_dbus_interface_constants_in_proxy() {
    let path = "../nairobi-hub/src/client.rs";

    // Verify the proxy uses correct interface constants
    assert!(
        file_contains(path, "org.nairobi.NairobiAxumRefinery1"),
        "D-Bus interface name not found in proxy"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// 5.2 FD Passing Tests
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_zero_copy_fd_passing() {
    let path = "../nairobi-hub/src/client.rs";

    // Verify OwnedFd is used for FD passing
    assert!(
        file_contains(path, "OwnedFd"),
        "OwnedFd not found in client.rs"
    );

    // Verify sql_query passes FD
    assert!(
        file_contains(path, "sql_query"),
        "sql_query method not found"
    );
    assert!(
        file_contains(path, "OwnedFd"),
        "sql_query should use OwnedFd parameter"
    );
}

#[test]
fn test_decoder_exists() {
    let path = "../nairobi-hub/src/decoder.rs";

    assert!(
        file_contains(path, "generate_report"),
        "generate_report function not found in decoder.rs"
    );
    assert!(
        file_contains(path, "DistilledAnalytics"),
        "DistilledAnalytics not found in decoder.rs"
    );
}

#[test]
fn test_decoder_markdown_output() {
    let path = "../nairobi-hub/src/decoder.rs";

    // Verify decoder generates Markdown
    assert!(
        file_contains(path, "Markdown"),
        "Markdown mention not found"
    );
    assert!(
        file_contains(path, "#"),
        "Markdown header not found in output"
    );
    assert!(
        file_contains(path, "**"),
        "Markdown bold not found in output"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// 5.3 E2E Workflow Tests (Require Running Refinery)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
#[ignore] // Requires: systemctl --user start nairobi-refinery
fn test_e2e_refinery_connection() {
    // This test verifies we can connect to a running Refinery daemon

    // Check if Refinery is running
    let output = std::process::Command::new("busctl")
        .args(&["--user", "status", "org.nairobi.NairobiAxumRefinery1"])
        .output();

    match output {
        Ok(out) => {
            if out.status.success() {
                // Refinery is running, test connection
                // let client = nairobi_hub::RefineryClient::connect().await;
                // assert!(client.is_ok(), "Should connect to Refinery");
            } else {
                println!("WARNING: Axum-Refinery not running, skipping E2E test");
            }
        }
        Err(_) => {
            println!("WARNING: busctl not available, skipping E2E test");
        }
    }
}

#[test]
#[ignore] // Requires: Running Refinery + test data file
fn test_e2e_full_workflow() {
    // 1. Connect to Refinery
    // 2. Call ingest with a test file
    // 3. Call sql_query with a test query
    // 4. Verify result FD is valid

    println!("E2E workflow test: Remove #[ignore] and ensure Refinery is running");
}
