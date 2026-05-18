[English](README.md) | [简体中文](README.zh-CN.md)

# Nairobi OS

## Overview
Nairobi OS is a high-performance, distributed data science infrastructure designed for extreme resource efficiency. It enables processing of massive datasets in constrained environments (Edge, IoT, Serverless) by leveraging a specialized Rust-based refinery daemon. By utilizing kernel-level features such as `io_uring`, `memfd`, and Huge Pages, Nairobi OS achieves sub-millisecond IPC overhead and zero-copy data pipelines.

## Key Features
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
4.  **Nairobi Python**: The high-level bridge. Provides a Pythonic interface to the Rust ecosystem.

```text
[ Data Source ] -> (io_uring/Huge Pages) -> [ Axum Refinery ]
                                                  |
                                          (D-Bus / memfd / iceoryx2)
                                                  |
                                          [ Nairobi Hub ]
                                             /        \
                             [ Nairobi Python ]    [ Lagos Vision ]
                                     |                    |
                            [ Jupyter Notebook ] <-> [ Visual Output ]
```

## Installation

### Prerequisites
- **Operating System**: Linux or WSL2 (Kernel 5.10+ required for `io_uring` and `memfd`).
- **Rust**: 1.70+
- **Python**: 3.10+
- **System Libraries**:
    ```bash
    sudo apt-get update && sudo apt-get install -y \
        build-essential \
        pkg-config \
        libdbus-1-dev \
        python3-dev \
        dbus-x11 \
        libosmesa6-dev \
        mesa-utils
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

4. **Install the Wheel**:
    ```bash
    pip install target/wheels/nairobi_os-0.3.1-py3-none-any.whl
    ```

## System Configuration (Contributor Guide)

### Huge Pages
The refinery engine prioritizes 1GB Huge Pages for zero-copy buffers. To enable these on your host:
```bash
echo 1 | sudo tee /sys/kernel/mm/hugepages/hugepages-1048576kB/nr_hugepages
```
*Note: If 1GB pages are unavailable, the engine will automatically fall back to Transparent Huge Pages (THP).*

### io_uring and SQPOLL
The `DiracEngine` uses `io_uring` with `SQPOLL` for maximum I/O throughput. `SQPOLL` typically requires elevated privileges (`CAP_SYS_ADMIN`) or a kernel configured with `IORING_SETUP_SQPOLL`. If the engine cannot initialize `SQPOLL`, it will fall back to standard `io_uring` mode.

## Usage
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

## Testing
Nairobi OS includes a comprehensive test suite covering Rust units, IPC integration, and Python bindings.

### Running All Tests
```bash
# Run Rust tests
cargo test --workspace

# Run Python integration tests
python3 test_nairobi.py
```

### Benchmarking
Detailed performance benchmarks can be run from the `nairobi-benchmarks` directory:
```bash
cd nairobi-benchmarks
pip install -r requirements.txt
python orchestration/benchmark_runner.py --workload workloads/workload_nba_pipeline.yaml
```

## Troubleshooting
- **D-Bus Connection Refused**: Ensure `dbus-daemon` is running. In headless environments, use `dbus-launch`.
- **Lagos Rendering Issues**: Lagos requires a valid GPU driver or OSMesa for software fallback. Verify with `glxinfo`.
- **Huge Page Allocation Failed**: Check `/proc/meminfo` to ensure enough huge pages are reserved by the kernel.

## License
This project is licensed under the **PolyForm Noncommercial License 1.0.0**. It is free for personal, educational, and research use.

---
© 2026 Kevin Chege. All Rights Reserved.
