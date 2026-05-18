[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md)

# Nairobi Hub

## Overview
Nairobi Hub is the central IPC (Inter-Process Communication) orchestrator of the Nairobi OS. It manages the coordination of file descriptors, D-Bus signals, and shared memory segments between the high-performance Rust refinery and its clients.

## Key Features
- **FD Proxying**: Safely passes `memfd` file descriptors via D-Bus using GVariant signatures.
- **Service Management**: Monitors and manages the `org.nairobi.NairobiAxumRefinery1` lifecycle.
- **Hybrid Data Plane**: Dynamically routes data through `iceoryx2` shared memory (for performance) or D-Bus (for compatibility).
- **Semantic Decoding**: Decodes raw binary analytics into human-readable reports and native Python structures.

## Architecture
The Hub is divided into several internal modules:
- `client.rs`: The D-Bus proxy client.
- `shm_subscriber.rs`: Handles `iceoryx2` shared memory subscriptions.
- `decoder.rs`: Converts GVariant results into Markdown and JSON.

## Usage
The Hub is primarily used as a library by `nairobi-python` to communicate with the refinery.

## Development
When modifying the Hub, ensure that any changes to the D-Bus interface are also reflected in `nairobi-protocol`.

## Testing
Integration tests for the Hub verify the full IPC round trip:
```bash
cargo test -p nairobi-hub
```

## License
This project is licensed under the **Apache License 2.0**.

---
© 2026 Kevin Chege. All Rights Reserved.
