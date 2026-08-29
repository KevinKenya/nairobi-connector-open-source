[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [Русский](README.ru.md) | [한국어](README.ko.md) | [Finnish](README.fi.md) | [Cantonese](README.yue.md) | [Français](README.fr.md) | [Nederlands](README.nl.md)

# Nairobi OS: High-Performance, Zero-Copy AI & Data Science Infrastructure

[![PyPI Version](https://img.shields.io/pypi/v/nairobi-os.svg)](https://pypi.org/project/nairobi-os/) [![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://github.com/KevinKenya/nairobi-connector-open-source/blob/main/LICENSE) [![System](https://img.shields.io/badge/Kernel-Linux_6.17_Native-orange.svg)]() [![Arch](https://img.shields.io/badge/Architecture-x86__64_/_ARM64-red.svg)]()

---

## Why This Exists

Nairobi OS is infrastructure for running AI and data-science pipelines on local hardware
without paying the overhead that Python's default tooling introduces at each layer:

1. **The Python Tax** — end-to-end memory copying, GIL contention, and interpreter overhead
   on data-heavy workloads.
2. **The Browser Tax** — rendering latency and communication overhead when agent tooling is
   built on top of browser-based interfaces for long-running, high-frequency interactions.
3. **The OS Kernel Bottleneck** — process scheduling and display-server overhead (Wayland vs.
   X11 context switching) that adds latency to desktop-automation workloads.

Nairobi OS is a Rust-based infrastructure stack — zero-copy data pipelines, hardware-native
execution, and a semantic (non-pixel) interface for desktop automation — built to remove these
overheads directly. This repository is the open-source core of that stack.


Reference hardware used in development and benchmarking:
```
Processor: AMD Ryzen 5 PRO 4650U (6 Cores, 12 Threads)
Graphics:  AMD Radeon RX Vega 6 iGPU
Memory:    32 GB RAM
Storage:   256 GB NVMe
```

---

## Adoption

Launched May 6, 2026. Current PyPI download statistics (sourced from the live
[ClickPy dashboard](https://clickpy.clickhouse.com/dashboard/nairobi-os), verifiable directly):

| Metric | Value |
|---|---|
| Total downloads (since launch) | 2700 |
| Top adopting countries | US, HK, CN, DE, JP |

---

## Key Features

- **Computer Use Without Pixels**: AT-SPI2 semantic interface with TOON compression for
  native desktop interaction — agents read a structured UI tree, not screenshots.
- **Zero-Copy Ingestion**: `io_uring` and 1GB Huge Pages for kernel-bypass data loading.
- **Hardware-Accelerated Visualization**: Low-latency Jupyter plotting via `lagos-lite`
  (wgpu/egui).
- **Vectorized Execution**: Polars query engine + Rayon multi-threaded pipelines.
- **Python API**: `SovereignFrame` — a Python-facing frame API backed by memory-mapped IPC,
  so data manipulation from Python doesn't pay a serialization cost to reach the Rust layer.
- **Canvas Pipelines**: Visual node-graph compiler with a native file picker and SQL query
  presets for building pipelines without writing glue code.

---

## Architecture

The repository is structurally split: the open-source core provides the fundamental
high-performance data-processing primitives; a separate closed-source repository
(`Sovereign-Systems-Lab`) contains multi-agent, high-availability, and industry-specific
extensions built on top of this core.

```
                             +---------------------------------------+
                             |         Nairobi Python API            |
                             +---------------------------------------+
                                                 |
                                [ GVariant over D-Bus / shared memory ]
                                                 |
                                                 v
                             +---------------------------------------+
                             |           Nairobi Hub                 |
                             +---------------------------------------+
                                                 |
               +---------------------------------+---------------------------------+
               |                                                                   |
               v                                                                   v
+------------------------------+                                    +------------------------------+
|     Axum Refinery (Data)     | <===[ Zero-Copy IPC / iceoryx2 ]==> |     Lagos Vision (Visual)    |
+------------------------------+                                    +------------------------------+
```

### Open Source Crate Workspace (`crates/`)

1. **`nairobi-axum-refinery`** — Rust daemon managing raw data ingestion, Rayon-parallelized
   statistics, and Polars-vectorized query execution.
2. **`nairobi-hub`** — Central IPC orchestrator; routes file descriptors and signals between
   clients and the refinery daemon.
3. **`lagos-lite`** — Local-only rendering engine using egui/wgpu hardware acceleration with
   zero-copy mmap data access. Requires a physical display.
4. **`nairobi-protocol`** — Shared protocol layer: GVariant serialization schemes, error
   types, and shared-memory layouts.
5. **`nairobi-python`** — The Python extension module, compiled via PyO3 and packaged with
   Maturin.
6. **`nairobi-canvas`** — Immediate-mode node-graph compiler with hardware-accelerated UI
   (wgpu/egui), including a native file picker and SQL query presets.

### Private Extensions (`Sovereign-Systems-Lab`, closed-source)

Enterprise-tier components — advanced MCP integration, hardware-bound security, and
industry-specific modules — are maintained in a private repository and licensed separately.
Details available on request.

### Capability Comparison

| Capability | Open Source Core (`crates/`) | Enterprise Suite (private) |
|---|---|---|
| Ingestion Engine | `mmap` / `copy_file_range` | `io_uring` + `SQPOLL` + 1GB Huge Pages |
| Statistical Analysis | Basic descriptive stats | Vectorized multi-pass skew/kurtosis, correlation |
| Query Engine | In-process Polars SQL | Distributed Apache Arrow / DataFusion cluster |
| IPC Mechanism | POSIX shared memory / D-Bus | Zero-copy `iceoryx2` shared-memory arenas |
| Visualization | Local Jupyter `anywidget` (display required) | Headless WebSocket / WebRTC / Wayland overlays |
| Security | Standard POSIX boundaries | Hardware-bound identity, chained forensic ledger |
| Authentication | None (local trusted user) | Hardware binding (TPM 2.0 / CPU ID), private PKI |
| Target Deployment | Single-node Linux | Distributed cloud / edge node |

---

## Installation & Setup

### Requirements

- **OS**: Linux (Ubuntu 22.04+ recommended) or WSL2 (x86_64).
- **Python**: Python 3.12 or newer (required for the PyPI binary wheel `nairobi-os`; PyPI currently ships cp312-manylinux2014_x86_64 only — no sdist, no macOS/Windows/ARM wheels).
- **GPU**: Vulkan, Metal, or Mesa/OpenGL driver (software fallback supported).
- **Rust**: Stable toolchain (only needed if building from source).

### Quick Install (PyPI)

```bash
pip install nairobi-os
```

### Build from Source

```bash
git clone https://github.com/KevinKenya/nairobi-connector-open-source.git
cd nairobi-connector-open-source

python3 -m venv .venv
source .venv/bin/activate
pip install maturin pyo3-build-config zbus anywidget traitlets pandas

chmod +x build_wheel.sh
./build_wheel.sh --release
```

This compiles the native daemons, copies them into the package directory, and builds a wheel
under `crates/nairobi-python/target/wheels/`.

---

## Usage

### 1. Data Analytics (In-Memory Pipeline)

```python
import nairobi_os as nb

# Start the background refinery daemon
nb.connect()

# Ingest a dataset via zero-copy memory pipe
frame = nb.read_csv("simulator/fndds_ingredient_nutrient_value.csv")

# Vectorized computation via the Rust refinery
profile = frame.crunch("value")
print(f"Mean: {profile['mean']:.4f}")
print(f"Std Dev: {profile['std_dev']:.4f}")

# Arbitrary SQL directly against the memory-mapped frame
subset = frame.query("SELECT * FROM dataset WHERE value > 50.0")

# Lagos-accelerated interactive plotting widget
subset.plot(column="value")
```

### 2. Canvas Visual Pipelines

```python
import nairobi_os as nb

dag_bytes = nb.canvas.open()

if dag_bytes:
    nb.canvas.execute(dag_bytes)
```

The canvas UI supports a native file-picker ingest node and SQL query presets (All Columns,
Single Column, Where Clause, Multi-Column) for rapid pipeline construction.

### 3. Computer Use Without Pixels (MCP)

Agents interact with the exposed MCP server tools rather than reading screenshots:

```
                    COMPUTER USE SEQUENCE

[ LLM Agent ]                                 [ Nairobi OS ]
      |                                             |
      |===> nairobi_find_window("Text Editor") ====>| (Locates target)
      |<=== Returns window ID & bounds =============|
      |                                             |
      |===> nairobi_get_ui_map() ==================>| (Generates TOON)
      |<=== Returns compressed markdown tree =======|
      |     "[ID: 12] Button: 'Save'"               |
      |                                             |
      |===> nairobi_interact(12, "click") =========>| (Executes action)
      |<=== Returns success status =================|
```

---

## System Tuning (Contributor Guide)

### 1GB Huge Pages

Nairobi OS uses 1GB Huge Pages to reduce TLB translation overhead on large datasets:

```bash
echo 1 | sudo tee /sys/kernel/mm/hugepages/hugepages-1048576kB/nr_hugepages
```

If the system can't allocate a 1GB page due to fragmentation, the engine falls back to
Transparent Huge Pages (THP) automatically.

### D-Bus Broker Configuration

For high-frequency signal environments, use `dbus-broker` rather than legacy `dbus-daemon`.

---

## Support

If Nairobi OS is useful in your data pipeline or agentic architecture, consider supporting
continued development:

[![Support Nairobi OS Development](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-PayPal-blue.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=chege.finance@gmail.com&item_name=Support+Nairobi+OS+Development)

Direct inquiries: aiwithafrica@gmail.com

---

## License

Apache License 2.0. Portions of the TOON format and bridge implementation are credited to
The TOON Authors.

---
© 2026 Kevin Chege. Sovereign Systems Lab, Nairobi, Kenya.
