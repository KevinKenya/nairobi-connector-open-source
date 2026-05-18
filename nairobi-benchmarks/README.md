# Nairobi Benchmark Suite

## Overview
The Nairobi Benchmark Suite is a rigorous performance evaluation framework designed to compare Nairobi OS against industry-standard data processing libraries (e.g., Pandas). It focuses on end-to-end latency, memory efficiency, and the impact of "Fused Analytical Strikes" on real-world workloads.

## Key Metrics
- **Ingestion Latency**: Time to load data from disk into memory-resident structures.
- **Compute Density**: Peak Resident Set Size (RSS) during heavy analytical loads.
- **Pipeline Throughput**: Total time for fused ingest-crunch-correlate operations.

## Installation

### Prerequisites
- Python 3.10+
- Nairobi OS (installed and configured)

### Setup
```bash
cd nairobi-benchmarks
pip install -r requirements.txt
```

## Running Benchmarks

### 1. Prepare Datasets
Generate synthetic datasets to test scaling:
```bash
# Generate 10M Row Dataset
python datasets/generators/generate_synthetic.py --type tall --output datasets/synthetic/tall_10m.csv
```

### 2. Run Workloads
Execute a specific benchmark workload:
```bash
python orchestration/benchmark_runner.py --workload workloads/workload_statistical_distillation.yaml --iterations 10
```

### 3. Analyze Results
Benchmark results are stored in JSON format and can be visualized using the included plotting tools:
```bash
python visualization/plot_scaling.py
```

## Methodology
The suite follows a "Hardware-First" benchmarking methodology, ensuring that:
- Cold and warm starts are measured separately.
- Kernel caches are cleared (where possible) between runs.
- All calculations are verified for mathematical identity (±1e-5) using `result_validator.py`.

## License
This suite is part of the Nairobi OS project and is licensed under the **PolyForm Noncommercial License 1.0.0**.

---
© 2026 Kevin Chege. All Rights Reserved.
