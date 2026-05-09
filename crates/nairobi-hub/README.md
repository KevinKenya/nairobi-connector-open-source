# Nairobi Hub: The IPC Orchestrator

The Hub serves as the central control plane for Nairobi OS. it coordinates the flow of data handles and D-Bus signals between the high-level Python bindings and the low-level Rust refinery.

## 🏗️ Responsibilities
- **FD Proxying**: Safely passing file descriptors via D-Bus using GVariant.
- **Service Discovery**: Managing the `org.nairobi.NairobiAxumRefinery1` life cycle.
- **Zero-Copy Subscription**: Subscribing to `iceoryx2` shared memory segments for high-frequency data streaming.

## 🛠️ Implementation
The Hub uses `zbus` for modern, asynchronous D-Bus communication and `iceoryx2` for the high-performance shared memory data plane.
