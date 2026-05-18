[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md)

# Nairobi Axum Refinery

## Overview
The Axum Refinery is the high-performance core of Nairobi OS. Written in Rust, it is designed to saturate modern hardware through kernel-bypass I/O and vectorized parallel analytics. It functions as a D-Bus service that manages the lifecycle of data ingested into anonymous memory file descriptors (`memfd`).

## Key Features
- **Dirac Ingestion Engine**: A 3-tier ingestion strategy using `io_uring` (Tier 1), `copy_file_range` (Tier 2), and `mmap` (Tier 3).
- **Axiom Crunch**: Vectorized statistical moment calculation (Mean, Variance, Skewness, Kurtosis) powered by Polars and Rayon.
- **Relational Strike**: Optimized Pearson and Spearman correlation calculation.
- **SQL Analytics**: Direct execution of SQL queries on memory-resident data using `polars-sql`.
- **Zero-Copy Data Plane**: Exposes analytical results via `iceoryx2` shared memory and D-Bus.

## Architecture
The refinery is structured into specialized engines:
- `DiracEngine`: Handles hardware-accelerated I/O.
- `AnalyzeEngine`: Performs statistical calculations and SQL execution.
- `DbusService`: Implements the `org.nairobi.NairobiAxumRefinery1` interface.

## Installation

### Prerequisites
- **Kernel**: Linux 5.10+ (WSL2 supported).
- **Dependencies**: `libdbus-1-dev`, `pkg-config`.
- **Huge Pages**: The engine performs best with 1GB Huge Pages enabled.
    ```bash
    echo 1 | sudo tee /sys/kernel/mm/hugepages/hugepages-1048576kB/nr_hugepages
    ```

### Build
```bash
cargo build --release -p nairobi-axum-refinery
```

## Development

### Kernel-Level Configuration
Contributors should be aware that `DiracEngine` attempts to use `IORING_SETUP_SQPOLL`. For this to work without root, you may need to adjust `/proc/sys/kernel/unprivileged_userns_clone` or run with `CAP_SYS_ADMIN`.

### Tutorial: Adding a New Statistical Metric
1.  **Define the Metric**: In `src/analyze.rs`, update the `StatisticalProfile` struct and its `compute` method.
2.  **Update Protocol**: Add the new field to the `DistilledAnalytics` struct in `crates/nairobi-protocol/src/types.rs`.
3.  **Export via D-Bus**: Ensure the D-Bus interface in `src/dbus_service.rs` correctly serializes the updated profile.

### Testing
The refinery uses `tokio::test` for asynchronous integration testing.
```bash
cargo test -p nairobi-axum-refinery
```

#### Mocking for Isolated Testing
You can test the `AnalyzeEngine` in isolation by creating a `memfd` manually and passing it to the engine, bypassing the D-Bus layer:
```rust
let opts = memfd::MemfdOptions::default();
let mfd = opts.create("test.csv")?;
// Write test data...
let engine = AnalyzeEngine::new()?;
let results = engine.analyze(mfd.into_fd(), "target_column")?;
```

## Troubleshooting
- **`io_uring` initialization failed**: Check if your kernel supports `io_uring` (`zgrep CONFIG_IO_URING /proc/config.gz`).
- **Huge Page allocation failed**: Ensure the host has enough contiguous memory available. Check `grep Huge /proc/meminfo`.

## License
This project is licensed under the **Apache License 2.0**.

---
© 2026 Kevin Chege. All Rights Reserved.
