[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [Русский](README.ru.md) | [한국어](README.ko.md)

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

## Support
If you find Nairobi OS useful, consider supporting the project:

[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-PayPal-blue.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=chege.finance@gmail.com&item_name=Support+Nairobi+OS+Development)

## License
This project is licensed under the **Apache License 2.0**.

---
© 2026 Kevin Chege. All Rights Reserved.
