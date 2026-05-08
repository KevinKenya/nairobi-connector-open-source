<!-- Author: Kevin Chege. Location: Nairobi -->

# Nairobi Benchmark Suite (Scaffold)

This repository contains the rigorous, academic-grade benchmarking suite for **Nairobi OS**. It is designed to prove the performance superiority of "Fused analytics execution" on large-scale structured data.

## 📁 Repository Structure
*   `engines/`: Standardized wrappers for Pandas, Polars, DuckDB, and Nairobi OS.
*   `orchestration/`: The core benchmarking logic, metrics collection, and validation.
*   `datasets/`: Scripts to download real-world data and generate synthetic scaling datasets.
*   `workloads/`: YAML definitions of the "WOW" benchmarks.
*   `visualization/`: Tools to generate latency and memory scaling curves.
*   `methodology.md`: The scientific ground rules for the benchmarks.

## 🚀 Quick Start (Scaffold Only)

### 1. Install Dependencies
```bash
pip install -r requirements.txt
```

### 2. Prepare Datasets
```bash
# Fetch real NBA data
./datasets/download_scripts/fetch_nba.sh

# Generate 10M Row Tall Dataset
python datasets/generators/generate_synthetic.py --type tall --output datasets/synthetic/tall_10m.csv

# Generate 1000 Column Wide Dataset
python datasets/generators/generate_synthetic.py --type wide --output datasets/synthetic/wide_1000c.csv
```

### 3. Run Benchmarks
```bash
# Run the Statistical Distillation workload
python orchestration/benchmark_runner.py --workload workloads/workload_statistical_distillation.yaml
```

### 4. Visualize Results
```bash
python visualization/plot_scaling.py
```

## 🔬 Validation
The `result_validator.py` ensures that all engines produce mathematically identical results (±0.00001). Any engine that fails the math check is automatically flagged.

## ⚖️ License
This benchmark suite is part of the Nairobi OS Open Source release.
