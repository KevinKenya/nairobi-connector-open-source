<!-- Author: Kevin Chege. Location: Nairobi -->

# Nairobi Benchmark Suite (Scaffold)

[![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/KevinKenya/nairobi-connector-open-source/blob/main/nairobi-benchmarks/nairobiOsBenchmarks.ipynb)

This repository contains the rigorous benchmarking suite for **Nairobi OS**. It is designed to evaluate the performance of "Fused analytics execution" against standard, unoptimized Pandas implementations.

## 📁 Repository Structure
*   `engines/`: Standardized wrappers for Pandas and Nairobi OS.
*   `orchestration/`: Core benchmarking logic, metrics collection, and validation.
*   `datasets/`: Scripts to download real-world data and generate synthetic scaling datasets.
*   `workloads/`: YAML definitions of analytical benchmarks (NBA statistics, correlation pipelines).
*   `visualization/`: Tools to generate latency and memory scaling curves.
*   `methodology.md`: The scientific ground rules and mathematical validation logic.

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
