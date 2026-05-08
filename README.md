# Nairobi OS: Heavy Iron Data Science Infrastructure

**Author**: Kevin Chege. Location: Nairobi  
**License**: PolyForm Noncommercial License 1.0.0

Nairobi OS is a distributed microservice architecture designed for high-performance, zero-copy data analysis. It enables processing of massive datasets in extreme resource-constrained environments (Edge, IoT, Serverless) by offloading heavy lifting to a specialized Rust-based refinery daemon.

## 🏗️ Architecture
Nairobi OS is built on a triad of specialized components connected via D-Bus:

1.  **[Nairobi Axum Refinery](crates/nairobi-axum-refinery/)**: The high-performance engine. Uses `io_uring` and 1GB Huge Pages for zero-copy ingestion and Rayon-parallelized Rust analytics.
2.  **[Nairobi Hub](crates/nairobi-hub/)**: The orchestrator. Manages `memfd` handles and provides a high-level client proxy for the Refinery.
3.  **[Nairobi Python](crates/nairobi-python/)**: The bridge. A PyO3-powered interface that brings Rust's performance to the Python ecosystem with sub-millisecond IPC overhead.
4.  **[Nairobi Protocol](crates/nairobi-protocol/)**: The constitution. Shared GVariant signatures and interface definitions that ensure type safety across the stack.

## 🚀 Key Innovation: The Fused Strike (v0.2.0)
Nairobi OS v0.2.0 introduces **Fused Pipeline Execution**. Traditionally, inter-process analytics suffer from multiple D-Bus round trips and redundant file parsing. The Fused Strike architecture eliminates this by executing the entire ingestion -> analysis -> correlation pipeline in a **single atomic D-Bus call**.

### Performance vs. Pandas
| Metric | Pandas | Nairobi OS (v0.2.0) |
|--------|--------|---------------------|
| **RAM Usage** | 4,285 MB | **20.5 MB** (209x lower) |
| **CPU Usage** | 1,158% | **10%** (116x lower) |
| **Pipeline Latency** | 912 ms | 1,160 ms (Competitive) |

## 🛠️ Installation

### Python Bindings
```bash
pip install nairobi-os
```

### Build from Source
```bash
# Clone the repository
git clone https://github.com/KevinKenya/nairobi-connector-open-source
cd nairobi-connector-open-source

# Build the entire stack
./build_wheel.sh
```

## 💻 Usage Example

```python
import nairobi_os
import json

# Start the Axum Refinery daemon
nairobi_os.start_refinery()

# Run a fused analytics strike
# (Ingest + Statistics + Correlation in one call)
result = json.loads(nairobi_os.data.pipeline(
    "dataset.csv", 
    "target_col", 
    "col1,col2"
))

print(f"Mean: {result['mean']}")
print(f"Pearson Correlation: {result['pearson']}")

# Stop the refinery
nairobi_os.stop_refinery()
```

## 📊 Benchmarking & Reports
We maintain a rigorous, academic-grade benchmarking suite.
- **[Benchmark Report](nairobi-benchmarks/BENCHMARK_REPORT.md)**: Comparison against Polars, Pandas, and DuckDB.
- **[Use Case Report](nairobi-benchmarks/NAIROBI_USE_CASE_REPORT.md)**: Guidance on when to deploy Nairobi OS.
- **[Stage Analysis](nairobi-benchmarks/PIPELINE_STAGE_REPORT.md)**: Deep dive into IPC vs Compute latency.

## ⚖️ License
This project is licensed under the **PolyForm Noncommercial License 1.0.0**. It is free for non-commercial use (Personal, Educational, Research). 

---
© 2026 Kevin Chege. All Rights Reserved.
