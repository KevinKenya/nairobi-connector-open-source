[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md)

# Nairobi OS

## Overview
Nairobi OS is a high-performance, distributed AI and data science infrastructure designed for extreme resource efficiency. It enables processing of massive datasets in constrained environments (Edge, IoT, Serverless) by leveraging a specialized Rust-based refinery daemon, and provides **Computer Use without pixels** via its MCP-compliant accessibility bridge.

By utilizing kernel-level features such as `io_uring`, `memfd`, and Huge Pages, Nairobi OS achieves sub-millisecond IPC overhead and zero-copy data pipelines.

## Key Features
- **Computer Use Without Pixels**: Interacts directly with the Linux desktop through AT-SPI2 and the TOON (Token-Oriented Object Notation) compression algorithm, bypassing OCR or visual processing for AI agents.
- **Zero-Copy Ingestion**: Hardware-accelerated data loading using `io_uring` and 1GB Huge Pages.
- **Hardware-Accelerated Visualization**: Interactive Jupyter plotting via the Lagos Vision engine (`wgpu` and `egui`).
- **Fused Analytics Pipeline**: Ingest, crunch, and correlate data in a single D-Bus round trip.
- **Kernel-Bypass Performance**: Vectorized analytics leveraging Polars and Rayon for maximum hardware saturation.
- **Sovereign Interface**: A fluent Python API that hides the complexity of low-level IPC and memory management.

## Architecture
Nairobi OS is built on a triad of specialized components connected via D-Bus and shared memory:

1.  **Nairobi Axum Refinery**: The high-performance Rust core. Manages raw data ingestion and parallelized analytics.
2.  **Nairobi Hub**: The IPC orchestrator. Coordinates file descriptors and signals between the refinery and clients.
3.  **Lagos Vision**: The visual cortex. A headless rendering engine that maps `memfd` handles directly into the GPU pipeline.
4.  **Nairobi Connector**: The semantic bridge. An MCP server exposing the Linux desktop's accessibility tree to LLMs.
5.  **Nairobi Python**: The high-level bridge. Provides a Pythonic interface to the entire Rust ecosystem.

## Installation

### From PyPI
```bash
pip install nairobi-os
```

### Build from Source
1. **Clone the Repository**:
    ```bash
    git clone https://github.com/KevinKenya/nairobi-connector-open-source
    cd nairobi-connector-open-source
    ```

2. **Setup Virtual Environment**:
    ```bash
    python3 -m venv .venv
    source .venv/bin/activate
    pip install maturin pyo3-build-config zbus anywidget traitlets
    ```

3. **Build the Entire Stack**:
    ```bash
    ./build_wheel.sh
    ```

## Usage

### Data Analytics
```python
import nairobi_os

# Ignite the refinery
nairobi_os.connect()

# Ingest data into a SovereignFrame
df = nairobi_os.read_csv("dataset.csv")

# Perform vectorized analytics
print(f"Mean: {df.column_name.mean()}")

# Spawn interactive visualization
df.plot()
```

### Computer Use (MCP Server)
AI agents using the Nairobi Connector should follow this fundamental loop via MCP tools:
1. Target a window using `nairobi_find_window`.
2. Observe the current state via `nairobi_get_ui_map`.
3. Read the TOON `[ID: N]` of the desired interactive element.
4. Execute an action on that element via `nairobi_interact` or `nairobi_type_text`.

## System Configuration (Contributor Guide)

### Huge Pages
The refinery engine prioritizes 1GB Huge Pages for zero-copy buffers. To enable these on your host:
```bash
echo 1 | sudo tee /sys/kernel/mm/hugepages/hugepages-1048576kB/nr_hugepages
```
*Note: If 1GB pages are unavailable, the engine will automatically fall back to Transparent Huge Pages (THP).*

### io_uring and SQPOLL
The `DiracEngine` uses `io_uring` with `SQPOLL` for maximum I/O throughput.

## Support
If you find Nairobi OS useful, consider supporting the project:

[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-PayPal-blue.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=chege.finance@gmail.com&item_name=Support+Nairobi+OS+Development)

## License
This project is licensed under the **Apache License 2.0**.
*(Note: Portions of the TOON format and bridge implementation are credited to The TOON Authors.)*

---
© 2026 Kevin Chege. All Rights Reserved.
