[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md)

# Lagos Vision (lagos-lite)

## Overview
Lagos Vision is the high-performance rendering engine for Nairobi OS. It is designed to visualize millions of data points with sub-millisecond latency by memory-mapping analytical data directly into the GPU pipeline. Lagos operates as a headless daemon that streams JPEG-encoded frames to Jupyter notebook widgets via WebSockets.

## Key Features
- **Zero-Copy Rendering**: Data is memory-mapped from `memfd` handles directly into `wgpu` buffers.
- **Hardware Acceleration**: Uses `egui` and `wgpu` (Vulkan, Metal, DX12, or OpenGL) for high-performance plotting.
- **LTTB Downsampling**: Implements the Largest-Triangle-Three-Buckets algorithm on the GPU to maintain visual accuracy while rendering massive datasets.
- **Event-Driven Architecture**: Consumes zero CPU when idle; only renders on data updates or user interaction.

## Architecture
Lagos Vision consists of:
- **Lagos Lite**: The core library providing the rendering pipeline.
- **Lagos Vision Daemon**: The binary process that manages the `wgpu` surface and WebSocket server.
- **Lagos Widget**: An `anywidget` Python component that displays the stream.

## Installation

### Prerequisites
- **GPU**: A Vulkan-compatible GPU (or OSMesa for software fallback).
- **System Libraries**: `libosmesa6-dev`, `mesa-utils`, `xvfb`.

### Build
```bash
cargo build --release -p lagos-lite --bin lagos-vision-daemon
```

## Usage

### In Nairobi OS
Lagos is typically used through the `SovereignFrame.plot()` method in Python.

### Manual Debugging
You can start the daemon manually to test the rendering pipeline:
```bash
./target/release/lagos-vision-daemon --fd <FD_INT> --width 1000 --height 400
```

## Development

### Implementing a Custom Visualization Layer
1.  **Modify the Pipeline**: In `src/pipeline.rs`, define your vertex and fragment shaders (WGSL).
2.  **Update the Buffer Layout**: Map the incoming `memfd` data to your new shader's bind groups.
3.  **UI Integration**: Add control elements (sliders, buttons) to the `egui` interface in `src/device.rs`.

### Headless Environments
In environments like Google Colab, Lagos uses `xvfb-run` or OSMesa to handle the lack of a physical display:
```bash
xvfb-run -s "-screen 0 1024x768x24" ./target/release/lagos-vision-daemon ...
```

## Testing
Lagos includes visual integration tests that capture frames and compare them against golden images.
```bash
cargo test -p lagos-lite
```

## Troubleshooting
- **WebSocket Connection Failed**: In cloud environments (Colab/SageMaker), ensure the proxy port is correctly mapped. Nairobi Python handles this automatically if `google.colab` is detected.
- **WGPU Adapter Not Found**: Ensure GPU drivers are installed. If using a CPU-only environment, Lagos will attempt to fall back to a software adapter.

## License
This project is licensed under the **PolyForm Noncommercial License 1.0.0**.

---
© 2026 Kevin Chege. All Rights Reserved.
