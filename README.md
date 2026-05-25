[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [Русский](README.ru.md) | [한국어](README.ko.md) | [Finnish](README.fi.md) | [Cantonese](README.yue.md) | [Français](README.fr.md)

# Nairobi OS: High-Performance, Zero-Copy AI & Data Science Infrastructure

[![PyPI Version](https://img.shields.io/pypi/v/nairobi-os.svg)](https://pypi.org/project/nairobi-os/)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)
![System](https://img.shields.io/badge/Kernel-Linux_6.17_Native-orange.svg)
![Arch](https://img.shields.io/badge/Architecture-x86__64_/_ARM64-red.svg)

---

## The Origin: From the Crucible to the Metal

Nairobi OS is not the product of a comfortable corporate incubator or a venture-backed research lab. It is the result of absolute necessity, born from a sequence of deep personal crises and a relentless drive to execute where standard industry tools fail.

I am Kevin Chege, 45 year old founder of Sovereign Systems Lab (Nairobi, Kenya). From 2009 to 2022, my life was consumed by severe alcoholism. It cost me professional standing, opportunities, and nearly my life. At the height of my addiction, I worked as an Analyst in the Strategy Office of The Open University in Milton Keynes, UK, following my time as the Founder and President of AIESEC in Rwanda (2006–2010). Today, I am in my fourth year of continuous sobriety.

```
                     LEGIO XIII GEMINA
              "The 13th Legion — June 13th"
     Thirteen years lost. Thirteen years to reclaim.
```

My programming journey is rooted in low-level systems architecture and extreme optimization. In 2015, I laid out my vision for building decentralized, highly technical capabilities on the African continent in [this treatise on Kenya's Silicon Valley](https://www.linkedin.com/pulse/building-kenyas-silicon-valley-making-work-kevin-chege/). When the LLM gold rush began in 2023, I was early. I built and deployed LLM wrappers, but quickly recognized their limitations, as documented in this early [2023 LLM wrapper demonstration](https://www.linkedin.com/feed/update/urn:li:activity:7102930955807449088/). 

I realized that building high-level wrappers on top of unstable APIs was a architectural dead-end. The real war is fought at the intersection of local hardware constraints and resource allocation.

Throughout 2025, I lived on a Lenovo X13 ThinkPad with a highly constrained hardware profile:

```
Processor: AMD Ryzen 5 PRO 4650U (6 Cores, 12 Threads)
Graphics: AMD Radeon RX Vega 6 iGPU
Memory: 32 GB RAM (with high system utilization)
Storage: 256 GB NVMe
```

On this exact machine, I spent 2025 building **Tumz** ([Sarafakai](http://www.sarafakai.com)), an air-gapped, zero-latency clinical decision support AI. It executed live, real-time audio transcription and clinical inference simultaneously on the integrated GPU (iGPU), keeping the entire Unified Medical Language System (UMLS) resident in RAM. We are currently partnering with a Kenyan hospital to pilot Tumz for a year-long clinical trial—because human health requires rigorous, empirical validation, not developers' assumptions.

During the development of Tumz, I encountered the massive, systemic inefficiencies of the modern data science stack:
1. **The Python Tax**: End-to-end memory copying, GIL bottlenecks, and massive runtime overhead.
2. **The Browser Tax**: Manifest V3 complications, rendering latency, and high-frequency communication failures in long-running agentic conversations.
3. **The OS Kernel Bottleneck**: Inefficient process scheduling, CPU thread starvation, and display server overhead (Wayland vs. X11 context switching).

So, at the close of 2025, I set out to build an infrastructure stack that bypasses these limits entirely—an Agentic Operating System designed for zero-copy data pipelines and hardware-native AI execution. This repository is the open-source core of that engine.

---

## Global Traction & Telemetry

Launched on May 6, 2026, Nairobi OS has rapidly gained traction among systems programmers, quantitative researchers, and edge computing architects worldwide. These download statistics are sourced from the live [ClickPy Nairobi OS Dashboard](https://clickpy.clickhouse.com/dashboard/nairobi-os), where you can search and explore the metrics for yourself.

### Cumulative Global Distribution (May 6, 2026 – May 23, 2026)

| Metric | Measurement | Context |
| :--- | :--- | :--- |
| **Global Rank** | **#75,293** | Out of 797,894 active packages on PyPI |
| **Percentile** | **9.43%** | Top-tier ranking for system-level Python extensions |
| **Total Downloads** | **1,525** | Clean, organic, high-intent developer downloads |

### Download Volume by Version

```
  0.2.0 [████████████████████████████████████████] 342
  0.2.1 [██████████████████████████] 224
  0.3.0 [████████████████████████] 212
  0.3.1 [████████████████████] 176
  0.1.0 [███████████████████] 169
  0.4.1 [██████████████] 120
```

### Top 10 Sovereign Regions of Adoption

| Rank | Region | Country Code | Download Volume |
| :--- | :--- | :--- | :--- |
| 1 | United States | US | 661 |
| 2 | Hong Kong | HK | 103 |
| 3 | China | CN | 84 |
| 4 | Germany | DE | 74 |
| 5 | Japan | JP | 65 |
| 6 | Singapore | SG | 56 |
| 7 | United Kingdom | GB | 51 |
| 8 | France | FR | 51 |
| 9 | Russia | RU | 42 |
| 10 | South Korea | KR | 30 |

---

## Support & Sovereignty

If Nairobi OS is optimizing your data pipelines, reducing your cloud bills, or driving your local agentic architectures, consider supporting our independent systems research. Every contribution is directly deployed into hardware-level compiler optimizations and edge-compute testing in Nairobi.

[![Support Nairobi OS Development](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-PayPal-blue.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=chege.finance@gmail.com&item_name=Support+Nairobi+OS+Development)

For direct inquiries, contact: aiwithafrica@gmail.com

For direct inquiries, contact: aiwithafrica@gmail.com

---

## Key Features

* **Computer Use Without Pixels**: Bypasses slow, expensive vision-based agent pipelines. Interacts natively with the Linux desktop through AT-SPI2 and the TOON (Token-Oriented Object Notation) compression algorithm, feeding raw hierarchical trees directly to LLMs.
* **Zero-Copy Ingestion**: Hardware-accelerated, kernel-bypass data loading utilizing `io_uring` and 1GB Huge Pages.
* **Hardware-Accelerated Visualization**: Low-latency, interactive Jupyter plotting using the `lagos-lite` rendering daemon, built on `wgpu` and `egui`.
* **Vectorized Analytical Execution**: Extreme CPU saturation utilizing Polars query execution and Rayon multi-threaded data pipelines.
* **Sovereign Interface**: A fluent Python API (`SovereignFrame`) that encapsulates memory-mapped file descriptors and IPC.

---

## Open Source vs. Enterprise Architecture

Nairobi OS is structurally bifurcated. The open-source repository provides the fundamental high-performance data processing and single-node visualization primitives. The closed-source commercial ecosystem contains the advanced multi-agent, high-availability, and industry-specific implementations.

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

1. **`nairobi-axum-refinery`**: High-performance Rust daemon managing raw data ingestion, Rayon-parallelized statistics, and Polars-vectorized query execution.
2. **`nairobi-hub`**: The central IPC orchestrator. Manages and routes file descriptors and signals between clients and the refinery daemon.
3. **`lagos-lite`**: The visual cortex. A headless, event-driven rendering engine that maps memory-mapped files directly into the GPU pipeline.
4. **`nairobi-protocol`**: The shared protocol layer. Defines standard GVariant serialization schemes, error types, and shared memory layouts.
5. **`nairobi-python`**: The Python extension module compiled via `PyO3` and packaged with `Maturin`.

### Private Corporate Ecosystem (`modules/`)

Our enterprise-tier components are held in a private repository (`Sovereign-Systems-Lab`) and licensed for industrial, financial, and state-level infrastructure.

1. **`sovereign-ui`**: The enterprise AT-SPI2 engine. Implements Aegis Protocol security, hardware binding, and production-grade desktop manipulation.
2. **`nairobi-connector`**: Advanced Model Context Protocol (MCP) server managing raw, low-latency D-Bus signals for enterprise LLMs.
3. **`tactical-rtos-node`**: Ultra-low-latency, real-time operating system scheduler for safety-critical edge industrial automation.
4. **`industrial-guardian-rust` / `industrial-guardian-python`**: Autonomous site reliability engineering (SRE) layer with predictive OOM, memory leak, and system crash avoidance.
5. **`fintech-bridge-rust`**: Real-time high-frequency transaction parser and legacy mainframe bridge (EBCDIC/SBA terminal parsing).
6. **`aviation-audio-rust`**: Sub-millisecond, lock-free audio stream processing, acoustic telemetry analysis, and raw wave DSP.
7. **`drawbridge_api`**: Secure, authenticated, multi-tenant gRPC drawbridge isolating the local kernel from untrusted cloud agent calls.

### Capability Comparison Matrix

| Capability / Feature | Open Source Core (`crates/`) | Enterprise Suite (`modules/`) |
| :--- | :---: | :---: |
| **Ingestion Engine** | `mmap` / `copy_file_range` | `io_uring` + `SQPOLL` + 1GB Huge Pages |
| **Statistical Analysis** | Basic descriptive stats | Vectorized, multi-pass skew/kurtosis, correlation |
| **Query Engine** | In-process Polars SQL | Distributed Apache Arrow / DataFusion cluster |
| **IPC Mechanism** | POSIX shared memory / D-Bus | Zero-Copy `iceoryx2` shared memory arenas |
| **Visualization** | Local Jupyter `anywidget` | WebRTC GStreamer / transparent Wayland Layer-Shell overlays |
| **Security & Compliance** | Standard POSIX boundaries | Aegis Protocol, SHA-256 Chained Forensic Ledger |
| **Authentication** | None (Local trusted user) | Hardware Binding (TPM 2.0 / CPU ID), private PKI |
| **Platform Target** | Single-node Linux | Distributed Cloud / Edge Node / High-Frequency Trading |

---

## Installation & Setup

### Requirements
- **OS**: Linux (Ubuntu 22.04+ recommended) or Windows Subsystem for Linux (WSL2).
- **GPU**: Vulkan, Metal, or OpenGL compatible driver.
- **Python**: 3.10 or newer.
- **Rust**: Stable toolchain (if building from source).

### Quick Install (PyPI)
```bash
pip install nairobi-os
```

### Build from Source
To compile the entire workspace, including the native daemons and Python extension:

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/KevinKenya/nairobi-connector-open-source.git
   cd nairobi-connector-open-source
   ```

2. **Configure Virtual Environment**:
   ```bash
   python3 -m venv .venv
   source .venv/bin/activate
   pip install maturin pyo3-build-config zbus anywidget traitlets pandas
   ```

3. **Execute Workspace Build**:
   ```bash
   chmod +x build_wheel.sh
   ./build_wheel.sh --release
   ```
   This compiles the native daemons, copies them to the package directory, and builds a wheel under `crates/nairobi-python/target/wheels/`.

---

## Usage Guide

### 1. Data Analytics (The In-Memory Pipeline)

Nairobi OS provides the `SovereignFrame` API. It handles raw memory mapping under the hood, enabling rapid data manipulation.

```python
import nairobi_os as nb

# Ignite the background refinery daemon
nb.connect()

# Ingest dataset using zero-copy memory pipe
frame = nb.read_csv("simulator/fndds_ingredient_nutrient_value.csv")

# Perform vectorized calculations via Rust refinery
profile = frame.crunch("value")
print(f"Mean: {profile['mean']:.4f}")
print(f"Std Dev: {profile['std_dev']:.4f}")

# Execute arbitrary SQL queries directly on the memory-mapped frame
subset = frame.query("SELECT * FROM dataset WHERE value > 50.0")

# Spawn the Lagos-accelerated interactive plotting widget
subset.plot(column="value")
```

### 2. Computer Use Without Pixels (MCP)

To use the AT-SPI2 semantic interface, your AI agent should interact with the exposed MCP server tools rather than reading screenshots:

```
                     COMPUTER USE SEQUENCE
                     
  [ LLM Agent ]                                 [ Nairobi OS ]
        |                                             |
        |===> nairobi_find_window("Text Editor") ====>| (Locates Target)
        |<=== Returns Window ID & Bounds =============|
        |                                             |
        |===> nairobi_get_ui_map() ==================>| (Generates TOON)
        |<=== Returns compressed Markdown Tree =======|
        |     "[ID: 12] Button: 'Save'"               |
        |                                             |
        |===> nairobi_interact(12, "click") =========>| (Executes Action)
        |<=== Returns Success Status =================|
```

---

## System Tuning (Contributor Guide)

To achieve the performance profiles shown in our benchmarks, your host kernel must be configured for system-level memory mapping.

### 1GB Huge Pages
Nairobi OS uses 1GB Huge Pages to bypass the CPU’s Translation Lookaside Buffer (TLB) translation overhead on massive datasets. 

To allocate a Huge Page on your Linux host:
```bash
echo 1 | sudo tee /sys/kernel/mm/hugepages/hugepages-1048576kB/nr_hugepages
```
*Note: If the system cannot allocate a 1GB page due to fragmentation, the engine automatically falls back to Transparent Huge Pages (THP).*

### D-Bus Broker Configuration
In high-frequency environments, ensure `dbus-broker` is installed instead of legacy `dbus-daemon` to handle rapid signal propagation across the control plane.

---

## License

This project is licensed under the **Apache License 2.0**.  
*(Note: Portions of the TOON format and bridge implementation are credited to The TOON Authors.)*

---
© 2026 Kevin Chege. All Rights Reserved.  
*Sovereign Systems Lab, Nairobi, Kenya.*
