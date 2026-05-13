# Nairobi Protocol: The GVariant Interface Definition

This crate defines the shared D-Bus interfaces and GVariant signatures used by Nairobi OS. It ensures mathematical and type-safety consistency across the Rust refinery, the Python bridge, and the Hub orchestrator.

**Version**: 0.3.1

## 🏗️ Interface Definition
- **Service Name**: `org.nairobi.NairobiAxumRefinery1`
- **Object Path**: `/org/nairobi/NairobiAxumRefinery1`
- **Interface**: `org.nairobi.NairobiAxumRefinery1`

## 🛠️ Key Methods

### Core Operations
- `Ingest(s file_path, s delimiter, s encoding)` -> `h memfd_handle` — Ingest a CSV file into a zero-copy memfd buffer.
- `Analyze(h memfd_handle, s query)` -> `v gvariant_result` — Perform moment-based statistical analysis on a column.
- `InspectSchema(h memfd_handle)` -> `v schema_inspection` — Inspect the schema of an ingested dataset.
- `CleanData(h memfd_handle, a(sss) strategies)` -> `h memfd_handle` — Apply cleaning strategies to a dataset.
- `SqlQuery(h memfd_handle, s query)` -> `h memfd_handle` — Execute a SQL query and return a new memfd handle.
- `Correlation(h memfd_handle, s query)` -> `v correlation_result` — Compute Pearson/Spearman correlation between two columns.

### Fused Operations (Single D-Bus Round Trip)
- `CrunchAndCorrelate(h memfd_handle, s column, s corr_columns)` -> `v fused_result` — Analytics + correlation in one call.
- `IngestCrunchCorrelate(s file_path, s delimiter, s encoding, s column, s corr_columns)` -> `v fused_result` — Full pipeline in one call.

## 📦 Module Structure
- `interface.rs` — D-Bus interface constants
- `types.rs` — GVariant-compatible types (`DistilledAnalytics`, `FusedAnalyticsResult`, `CorrelationResult`, etc.)
- `error.rs` — Shared `ImperialError` enum
- `mem_pipe.rs` — `MemoryPipe` zero-copy memfd wrapper
- `arena.rs` — iceoryx2 shared memory arena types (`ArenaHeader`, `PayloadType`)

## ⚖️ Licensing
This project is licensed under the **PolyForm Noncommercial License 1.0.0**. It is free for personal, educational, and research use.

---
© 2026 Kevin Chege. All Rights Reserved.