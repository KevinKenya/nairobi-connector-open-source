# Nairobi OS: Heavy Iron Data Science Infrastructure

[![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/KevinKenya/nairobi-connector-open-source/blob/main/nairobi-benchmarks/nairobiOsBenchmarks.ipynb)

> [!IMPORTANT]
> **Linux & WSL2 Only.** We do not compromise on kernel physics.


**Author**: Kevin Chege. Location: Nairobi  
**License**: PolyForm Noncommercial License 1.0.0

## Thankyou to the rust community on Reddit for challenging the validity of my claims. This version has improved on those deficiencies. 
Thanks especially to SkiFire13. 

Nairobi OS is a distributed microservice architecture designed for high-performance, zero-copy data analysis. It enables processing of massive datasets in extreme resource-constrained environments (Edge, IoT, Serverless) by offloading heavy lifting to a specialized Rust-based refinery daemon.

## 🏗️ Architecture
Nairobi OS is built on a triad of specialized components connected via D-Bus:

1.  **[Nairobi Axum Refinery](crates/nairobi-axum-refinery/)**: The high-performance Rust core. Uses `io_uring` and 1GB Huge Pages for zero-copy ingestion and Rayon-parallelized analytics.
2.  **[Nairobi Hub](crates/nairobi-hub/)**: The IPC orchestrator. Manages `memfd` handles and provides high-level client proxies.
3.  **[Nairobi Python](crates/nairobi-python/)**: The high-level bridge. A PyO3-powered interface that brings Rust's performance to the Python ecosystem with sub-millisecond IPC overhead.
4.  **[Nairobi Protocol](crates/nairobi-protocol/)**: The shared GVariant signatures and interface definitions ensuring cross-crate type safety.

## 🚀 Key Innovation: The Fused Strike (v0.2.1)
Nairobi OS v0.2.1 introduces **Fused Pipeline Execution**. By executing the entire ingestion -> analysis -> correlation pipeline in a single atomic D-Bus call, we eliminate redundant inter-process round trips and file re-parsing.

### 📊 Performance Benchmark (v0.2.1)
| Metric | Pandas (Unoptimized) | Nairobi OS (v0.2.1) | Speedup |
|--------|--------|---------------------|---------|
| **Ingestion Latency** | 6.38s | **0.52s** | **12.2x** |
| **Statistical Distillation** | 0.04s | **1.59s** | **pandas significantly faster due to numpy C backend** |
| **Total Pipeline** | 6.42s | **2.29s** | **2.8x** |

## 🛠️ Installation

### Python Bindings
```bash
pip install nairobi-os==0.2.1
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
We maintain a rigorous, academic-grade benchmarking suite to validate our "Hardware-First" approach.
- **[Benchmark Report (v0.2.1)](nairobi-benchmarks/BENCHMARK_REPORT.md)**: Latest comparison against standard Pandas baselines.
- **[Methodology](nairobi-benchmarks/methodology.md)**: Our scientific ground rules for fair comparison.

## ⚖️ License
This project is licensed under the **PolyForm Noncommercial License 1.0.0**. It is free for non-commercial use (Personal, Educational, Research). 

---
© 2026 Kevin Chege. All Rights Reserved.
