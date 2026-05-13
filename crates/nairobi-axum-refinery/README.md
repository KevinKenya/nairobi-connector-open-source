# Nairobi Axum Refinery: High-Performance Rust Core

**Version**: 0.3.1

The Refinery is the "Heavy Iron" of Nairobi OS. It is a high-performance daemon written in Rust, designed to saturate modern hardware for forensic data analysis.

## 🚀 Key Technologies
- **io_uring**: High-performance asynchronous I/O for disk-bound ingestion.
- **Huge Pages**: Utilizing 1GB Huge Pages for zero-copy memory buffers.
- **Rayon**: Parallelizing moment-based statistical calculations across all available CPU cores.
- **Polars**: Leveraging the state-of-the-art vectorized analytical engine.
- **SafeMmap**: Custom RAII-based file descriptor management to ensure memory and FD safety.
- **iceoryx2**: Zero-copy shared memory publish-subscribe for the data plane.

## 🏗️ Internal Components
- **DiracEngine**: The low-level I/O orchestrator for huge pages and `io_uring`. Implements a 3-tier ingestion strategy:
  1. **Tier 1**: io_uring Read into Huge Page → write to memfd (hardware DMA path)
  2. **Tier 2**: `copy_file_range` kernel splice → memfd
  3. **Tier 3**: mmap fallback → memfd
- **AnalyzeEngine**: The statistical cortex performing mean, skewness, kurtosis, quantile, and correlation calculations using Polars + Rayon.
- **ShmPublisher**: The zero-copy data plane powered by `iceoryx2`, publishing results to a 64MB POSIX shared memory arena.
- **AxumRefineryService**: The D-Bus interface handler, routing requests to the appropriate engine and publishing results via iceoryx2 when available.

## 🔧 Build
```bash
cargo build --release -p nairobi-axum-refinery
```

## 🚀 Run
```bash
./target/release/nairobi-axum-refinery
```

The daemon will register on D-Bus as `org.nairobi.NairobiAxumRefinery1` at `/org/nairobi/NairobiAxumRefinery1`.

## 📂 Source Layout
- `src/main.rs` — Entry point: initializes tracing, service state, and D-Bus connection
- `src/lib.rs` — Module declarations (re-exports submodules)
- `src/dbus_service.rs` — D-Bus interface implementation (`#[dbus_interface]`)
- `src/ingest.rs` — `DiracEngine` for 3-tier zero-copy ingestion
- `src/analyze.rs` — `AnalyzeEngine` for vectorized statistics, SQL, and correlation
- `src/shm_publisher.rs` — `ShmPublisher` for iceoryx2 shared memory data plane

## ⚖️ Licensing
This project is licensed under the **PolyForm Noncommercial License 1.0.0**. It is free for personal, educational, and research use.

---
© 2026 Kevin Chege. All Rights Reserved.