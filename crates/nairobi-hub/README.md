# Nairobi Hub: The IPC Orchestrator

**Version**: 0.3.1

The Hub serves as the central control plane for Nairobi OS. It coordinates the flow of data handles and D-Bus signals between the high-level Python bindings and the low-level Rust refinery.

## 🏗️ Responsibilities
- **FD Proxying**: Safely passing file descriptors via D-Bus using GVariant.
- **Service Discovery**: Managing the `org.nairobi.NairobiAxumRefinery1` life cycle.
- **Zero-Copy Subscription**: Subscribing to `iceoryx2` shared memory segments for high-frequency data streaming.
- **Semantic Decoding**: Converting raw analytical results into human-readable Markdown reports.

## 🛠️ Implementation
The Hub uses `zbus` for modern, asynchronous D-Bus communication and `iceoryx2` for the high-performance shared memory data plane.

## 📦 Module Structure
- `client.rs` — D-Bus proxy client with automatic iceoryx2 data plane routing
- `decoder.rs` — Markdown report generators for analytics and correlation results
- `shm_subscriber.rs` — iceoryx2 subscriber + POSIX shm reader for zero-copy data reads

## 🔑 Key Design Decisions

### Hybrid Data Plane
The Hub automatically routes responses through the most efficient path:
1. **iceoryx2 path**: When the Refinery signals `"SHM_READY"`, the Hub reads the result directly from a POSIX shared memory arena — zero kernel copies, nanosecond latency.
2. **D-Bus fallback**: If iceoryx2 is unavailable, the Hub accepts the JSON payload directly from D-Bus.

### Iceoryx2 Architecture
```
Refinery (Publisher)          Hub (Subscriber)
┌──────────────────┐          ┌──────────────────┐
│ shm_publisher.rs │───iceoryx2──▶│ shm_subscriber.rs│
│ POSIX /dev/shm   │  header    │ POSIX /dev/shm   │
│ Arena (64MB)     │──▶read────▶│ Arena (mapped RO) │
└──────────────────┘          └──────────────────┘
```

## ⚖️ Licensing
This project is licensed under the **PolyForm Noncommercial License 1.0.0**. It is free for personal, educational, and research use.

---
© 2026 Kevin Chege. All Rights Reserved.