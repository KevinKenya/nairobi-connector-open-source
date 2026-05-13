# Lagos Vision: The Visual Cortex of Nairobi OS

**Version**: 0.3.1

Lagos Vision is a hardware-accelerated, event-driven rendering engine designed for high-performance data visualization in Jupyter environments. It adheres to the **Zero-Copy Doctrine**, memory-mapping distilled analytical data directly from `memfd` handles into the GPU pipeline.

## 🚀 Physics of Performance

- **Zero-Copy Ingestion**: Raw data never enters the Python interpreter. Lagos maps the refinery's output directly into its process space.
- **Hardware Acceleration**: Uses `egui` and `wgpu` to render millions of points via Vulkan, Metal, or OpenGL.
- **Event-Driven Rendering**: 0% CPU usage when idle. Lagos only renders when data changes or user interaction occurs.
- **LTTB Downsampling**: Dynamically decimates massive datasets into visually accurate representations for sub-millisecond interactivity.

## 🛠️ Components

1. **Lagos Lite (Library)**: The core rendering engine, providing the `SovereignStream` and `LagosPipeline`.
2. **Lagos Vision Daemon (Binary)**: A specialized background process spawned by the Python orchestrator to manage the rendering loop and WebSocket communication.
3. **Lagos Widget (Jupyter)**: An `anywidget`-powered interface that streams JPEG frames from the daemon to an HTML5 canvas.

## 💻 Manual Execution

While Lagos is typically orchestrated by `nairobi_os`, it can be run manually for debugging:

```bash
# Compile the daemon
cargo build --release -p lagos-lite --bin lagos-vision-daemon

# Run the daemon (requires a memfd handle ID)
./target/release/lagos-vision-daemon --fd <FD_INT> --width 1000 --height 400
```

The daemon will output `[LAGOS_PORT: XXXX]` on stdout once the WebSocket server is live.

## 📊 Performance Benchmark (v0.3.1)

| Metric | Standard (Matplotlib) | Lagos Vision | Speedup |
|--------|-----------------------|--------------|---------|
| **Latency (10M pts)** | ~12.5s (blocking) | **~0.015s (async)** | **800x** |
| **Idle CPU Load** | 5-10% (Polling) | **0.0% (Event-driven)** | **∞** |
| **Memory usage (10M pts)** | ~850MB | **< 40MB (Zero-Copy)** | **21x** |

> [!NOTE]
> Lagos achieves these speeds by offloading downsampling (LTTB) to the Rust core and utilizing `wgpu` for parallelized GPU rendering. Standard pipeline latency includes the mandatory data copy into the Python interpreter, which Lagos bypasses entirely.

## 🛠️ Build
```bash
cargo build --release -p lagos-lite --bin lagos-vision-daemon
```

## ⚖️ Licensing
This project is licensed under the **PolyForm Noncommercial License 1.0.0**. It is free for personal, educational, and research use.

---
© 2026 Kevin Chege. Location: Nairobi.