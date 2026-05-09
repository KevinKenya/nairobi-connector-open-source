# Nairobi Axum Refinery: High-Performance Rust Core

The Refinery is the "Heavy Iron" of Nairobi OS. It is a high-performance daemon written in Rust, designed to saturate modern hardware for forensic data analysis.

## 🚀 Key Technologies
- **io_uring**: High-performance asynchronous I/O for disk-bound ingestion.
- **Huge Pages**: Utilizing 1GB Huge Pages for zero-copy memory buffers.
- **Rayon**: Parallelizing moment-based statistical calculations across all available CPU cores.
- **Polars**: Leveraging the state-of-the-art vectorized analytical engine.
- **SafeMmap**: Custom RAII-based file descriptor management to ensure memory and FD safety.

## 🏗️ Internal Components
- **DiracEngine**: The low-level I/O orchestrator for huge pages and `io_uring`.
- **AnalyzeEngine**: The statistical cortex performing mean, skewness, and kurtosis calculations.
- **ShmPublisher**: The zero-copy data plane powered by `iceoryx2`.
