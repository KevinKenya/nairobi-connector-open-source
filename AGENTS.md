# Nairobi OS Development Guide

## Build

```bash
./build_wheel.sh --release
```
Builds all Rust microservices in parallel then packages Python wheel via maturin. Use `--release` for production builds.

## Workspace Crates

- `crates/nairobi-axum-refinery` — Data service daemon (io_uring, Polars SQL, Rayon)
- `crates/nairobi-hub` — IPC orchestrator with Decoder for semantic UI trees
- `crates/lagos-lite` — Hardware-accelerated rendering (wgpu/egui), binary: `lagos-vision-daemon`
- `crates/nairobi-connector` — AT-SPI2/MCP server for desktop automation, binary: `nairobi-connector`
- `crates/nairobi-protocol` — Shared GVariant/D-Bus types
- `crates/nairobi-canvas` — Immediate-mode node graph compiler
- `crates/nairobi-python` — PyO3 extension (lib name: `nairobi_os._core`)

## Testing

```bash
cargo test --package nairobi-axum-refinery
cargo test --package nairobi-hub
cargo test --package nairobi-protocol
cargo test --package nairobi-canvas
```
Tests use `memfd` for zero-copy memory mapping; no file fixtures required.

## Binary Entrypoints

- `nairobi-axum-refinery` — D-Bus service for data ingestion/query
- `nairobi-hub` — Orchestrator daemon
- `lagos-vision-daemon` — Visual rendering daemon
- `nairobi-connector` — MCP server for AT-SPI2 desktop automation

## Runtime Requirements

- Linux (Ubuntu 22.04+ recommended) with physical display
- Vulkan/Metal/OpenGL driver (hardware-accelerated rendering required)
- `dbus-broker` preferred over `dbus-daemon` for high-frequency signals
- Optional: 1GB huge pages via `echo 1 | sudo tee /sys/kernel/mm/hugepages/hugepages-1048576kB/nr_hugepages` (falls back to THP)

**Note**: `lagos-vision-daemon` (in `lagos-lite`) renders fully offscreen via `wgpu` with no window created (confirmed in `crates/lagos-lite/src/main.rs` and `src/device.rs` `HeadlessContext`), and is already invoked headlessly by `nairobi-hub/src/executor.rs` during DAG execution. Desktop automation features (such as MCP/AT-SPI2 accessibility bridge) require an active X11/Wayland desktop session and D-Bus accessibility bus, whereas the rendering and data processing pipelines support headless offscreen execution.

## Key Dependencies

- `zbus` 5.x with `gvariant` feature for D-Bus protocol
- `iceoryx2` 0.3.0 for zero-copy IPC
- `rmcp` 1.5 with `server`, `transport-io`, `macros` features