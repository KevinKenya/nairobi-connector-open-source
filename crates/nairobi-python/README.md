# Nairobi OS: Heavy Iron AI Infrastructure

> [!IMPORTANT]
> **Linux & WSL2 Only.** We do not compromise on kernel physics.

**Author**: Kevin Chege. Location: Nairobi
**License**: PolyForm Noncommercial License 1.0.0

Nairobi OS is a high-performance data science operating system primitive designed for extreme resource efficiency. It enables memory-constrained environments (Edge, Containers, Serverless) to process large-scale datasets with **hardware-accelerated ingestion** and vectorized Rust analytics.

## 🚀 Version 0.3.1: The Fused Strike
This release introduces the **Fused Analytics Pipeline**, allowing ingestion, statistical distillation, and correlation to happen in a single, high-speed D-Bus round trip.

### Key Features:
- **Fused Pipeline**: `nairobi_os.data.pipeline()` for maximum throughput.
- **Zero-Copy Ingestion**: Powered by `memfd`, `io_uring`, and kernel `copy_file_range`.
- **Extreme Speed**: Ingest 450MB+ datasets **12x faster** than standard Pandas.
- **Rayon Parallelization**: Vectorized analytics leveraging multi-core hardware saturation.
- **Persistent Infrastructure**: Cached D-Bus connections for low-latency calls.
- **Lagos Vision**: Hardware-accelerated Jupyter plotting via zero-copy `mmap` and `wgpu`.

## 🛠️ Installation

### From Wheel
```bash
pip install nairobi-os==0.3.1
```

### Build from Source
```bash
# Clone the repository
git clone https://github.com/chege/nairobi-connector-open-source
cd nairobi-connector-open-source

# Set up Python environment
python3 -m venv .venv
source .venv/bin/activate
pip install maturin pyo3-build-config zbus anywidget traitlets

# Build the entire stack
./build_wheel.sh

# Install the forged wheel
pip install target/wheels/nairobi_os-0.3.1-py3-none-any.whl
```

Or install in development mode:
```bash
cd crates/nairobi-python
pip install -e .
```

## 💻 Quick Start
```python
import nairobi_os
import json

# Ignite the refinery daemon
nairobi_os.start_refinery()

# Run a fused analytics strike
# (Ingest + Mean/StdDev/Skewness/Kurtosis + Pearson Correlation)
result_json = nairobi_os.data.pipeline(
    "data.csv",
    "target_column",
    "col1,col2" # Correlation pair
)

result = json.loads(result_json)
print(f"Mean: {result['mean']}")
print(f"Correlation: {result['pearson']}")

# Shutdown refinery
nairobi_os.stop_refinery()
```

## 📊 Performance Benchmark (v0.3.1)
| Metric | Pandas (Unoptimized) | Nairobi OS | Speedup |
|--------|--------|------------|---------|
| **Ingestion Latency** | 6.38s | **0.52s** | **12.2x** |
| **Statistical Distillation** | 1.04s | **0.02s** | **52x** |
| **Total Pipeline** | 6.42s | **2.29s** | **2.8x** |

## 📦 Python API Reference

### `nairobi_os.start_refinery(binary_path=None, timeout=15)`
Starts the Nairobi Axum Refinery daemon. Auto-discovers the binary from the `bin/` directory or falls back to `target/release/`.

### `nairobi_os.stop_refinery()`
Terminates the refinery daemon.

### `nairobi_os.connect()`
Auto-configures `XDG_RUNTIME_DIR`, starts D-Bus (if needed on headless/Colab), and ignites the refinery.

### `nairobi_os.read_csv(path, delimiter=",", encoding="utf-8")`
Ingests data and returns a `SovereignFrame`. Auto-starts refinery if offline.

### `nairobi_os.data.ingest(file_path)`
Ingest a CSV file. Returns a handle ID (UUID string).

### `nairobi_os.data.crunch(handle_id, column)`
Compute statistical moments on a column. Returns JSON string.

### `nairobi_os.data.correlate(handle_id, columns)`
Compute Pearson/Spearman correlation. `columns` is a comma-separated string. Returns JSON string.

### `nairobi_os.data.pipeline(file_path, column, corr_columns)`
Fused ingest + crunch + correlate in a single D-Bus round trip. Returns JSON string.

### `nairobi_os.data.sql_query(handle_id, query)`
Execute a SQL query on an ingested dataset. Returns a new handle ID.

### `nairobi_os.data.free(handle_id)`
Release a memfd handle.

### `nairobi_os.lagos.plot_inline(handle_id, width=1000, height=400)`
Spawn the Lagos Vision daemon and return an interactive Jupyter widget.

## ⚖️ Licensing
This project is licensed under the **PolyForm Noncommercial License 1.0.0**. It is free for personal, educational, and research use. For commercial inquiries, please contact the author.

---
© 2026 Kevin Chege. All Rights Reserved.