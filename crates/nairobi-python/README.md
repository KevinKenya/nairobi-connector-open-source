[English](README.md) | [简体中文](README.zh-CN.md)

# Nairobi Python

## Overview
Nairobi Python provides the high-level bridge to the Nairobi OS infrastructure. It enables data scientists to harness the power of Rust-based, hardware-accelerated analytics through a familiar, Pythonic interface. The package handles daemon management, IPC coordination, and memory-mapping, allowing users to focus on data analysis.

## Key Features
- **SovereignFrame**: A fluent, Pandas-like interface for managing remote data handles.
- **Lazy Ignition**: Automatically starts and configures the refinery daemon upon first data access.
- **Jupyter Integration**: First-class support for interactive visualizations using the Lagos Vision widget.
- **Zero-Copy Bridge**: Directly consumes `memfd` handles from the Rust refinery with sub-millisecond overhead.

## Installation

### From PyPI
```bash
pip install nairobi-os
```

### From Source
```bash
cd crates/nairobi-python
pip install -e .
```
*Note: Building from source requires the Rust toolchain and `maturin` to be installed.*

## Usage

### Quick Start
```python
import nairobi_os

# Connect to the refinery (automatically handles D-Bus and daemon startup)
nairobi_os.connect()

# Ingest a CSV file
df = nairobi_os.read_csv("data.csv")

# Fluent API for statistics
mean_val = df.column_name.mean()
p99_val = df.column_name.p99()

# Run SQL queries directly on the engine
tall_players = df.query("SELECT * FROM dataset WHERE height > 80")

# Plot using Lagos Vision
tall_players.plot()
```

## API Reference

### `nairobi_os.connect()`
Initializes the environment, starts the D-Bus session if necessary, and ignites the refinery daemon.

### `nairobi_os.read_csv(path, delimiter=",", encoding="utf-8")`
Ingests a CSV file using the refinery's zero-copy pipeline. Returns a `SovereignFrame`.

### `SovereignFrame` Methods
- `df.column.mean()`: Compute the arithmetic mean.
- `df.column.std_dev()`: Compute the standard deviation.
- `df.column.p95()`, `df.column.p99()`: Compute percentiles.
- `df.column.skewness()`, `df.column.kurtosis()`: Compute statistical moments.
- `df.query(sql_string)`: Execute Polars-SQL on the dataset.
- `df.correlate("col1,col2")`: Compute Pearson and Spearman correlation.
- `df.plot(width, height)`: Display an interactive `anywidget` visualization.

## Development

### Adding New Python Bindings
Nairobi Python uses PyO3 to interface with Rust. New core functions should be added to `crates/nairobi-python/src/lib.rs` and exposed through the `nairobi_os._core.data` module.

### Testing
Integration tests for the Python package can be run using `pytest` (if configured) or the provided test script:
```bash
python3 test_nairobi.py
```

To test in isolation without the full refinery, you can mock the `_core.data` module or use the `SovereignFrame` with pre-existing handles.

## Troubleshooting
- **Refinery Failed to Register on D-Bus**: This often happens in headless environments. Ensure `dbus-launch` is available or call `nairobi_os.connect()` which attempts to fix the environment.
- **Handle Not Found**: Data handles are session-bound. If the refinery restarts, previous `SovereignFrame` handles will become invalid.

## License
This project is licensed under the **PolyForm Noncommercial License 1.0.0**.

---
© 2026 Kevin Chege. All Rights Reserved.
