[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md)

# Nairobi Protocol

## Overview
The Nairobi Protocol crate defines the shared D-Bus interfaces, GVariant signatures, and data structures used throughout the Nairobi OS ecosystem. It serves as the "source of truth" for type safety across the Rust core, the Hub orchestrator, and the Python bindings.

## Key Components
- **Interface Definitions**: Constants for service names, object paths, and method signatures.
- **Shared Types**: GVariant-compatible structs such as `DistilledAnalytics` and `CorrelationResult`.
- **Memory Management**: The `MemoryPipe` wrapper for `memfd` operations and the `iceoryx2` arena definitions.

## D-Bus Interface
- **Service Name**: `org.nairobi.NairobiAxumRefinery1`
- **Object Path**: `/org/nairobi/NairobiAxumRefinery1`
- **Interface**: `org.nairobi.NairobiAxumRefinery1`

## Usage
Add this crate as a dependency in any component that needs to communicate within the Nairobi OS ecosystem.

## Development
Changes to this crate should be made with extreme care, as they require re-compilation of all dependent crates and may break binary compatibility between the refinery and the Python bindings.

## Testing
Integration tests ensure that the GVariant signatures match the expected D-Bus protocol:
```bash
cargo test -p nairobi-protocol
```

## License
This project is licensed under the **PolyForm Noncommercial License 1.0.0**.

---
© 2026 Kevin Chege. All Rights Reserved.
