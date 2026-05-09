# Nairobi Protocol: The GVariant Interface Definition

This crate defines the shared D-Bus interfaces and GVariant signatures used by Nairobi OS. It ensures mathematical and type-safety consistency across the Rust refinery, the Python bridge, and the Hub orchestrator.

## 🏗️ Interface Definition
- **Service Name**: `org.nairobi.NairobiAxumRefinery1`
- **Object Path**: `/org/nairobi/NairobiAxumRefinery1`
- **Interface**: `org.nairobi.NairobiAxumRefinery1`

## 🛠️ Key Methods
- `Ingest(fd)` -> `handle`: Accepts a file descriptor (`memfd` or standard file) and returns a handle for future operations.
- `Crunch(handle, column)` -> `json`: Performs moment-based statistical analysis on the specified column.
- `Correlate(handle, columns)` -> `json`: Computes Pearson/Spearman correlation between two columns.
- `Pipeline(fd, column, corr_columns)` -> `json`: Fused execution of the entire pipeline in a single D-Bus round trip.
