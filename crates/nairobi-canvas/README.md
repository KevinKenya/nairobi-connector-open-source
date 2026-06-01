# Nairobi Canvas: Immediate-Mode Node Graph Visual Compiler

Nairobi Canvas is a hardware-accelerated visual compiler for building data processing pipelines. It provides an immediate-mode node graph UI built on `egui`/`egui-snarl` that compiles visual workflows into GVariant DAG (Directed Acyclic Graph) format for execution by the Nairobi Hub.

## Features

- **Visual Pipeline Builder**: Drag-and-drop node graph interface for data workflows
- **Native File Picker**: Click the 📂 button on Ingest nodes to browse for CSV files
- **SQL Query Presets**: Pre-configured query templates (All Columns, Single Column, Where Clause, Multi-Column)
- **GVariant Serialization**: Compiles graphs to GVariant format for zero-copy IPC
- **Topological Sort**: Automatic cycle detection and execution ordering

## Node Types

| Node | Inputs | Outputs | Description |
|------|--------|---------|-------------|
| **Ingest** | 0 | 1 | Loads CSV datasets via native file picker |
| **SqlQuery** | 1 | 1 | Executes Polars SQL queries on input data |
| **AxiomCrunch** | 1 | 1 | Computes statistics (mean, std_dev, kurtosis) |
| **LagosPlot** | 1 | 0 | Renders visualizations (sparkline, scatter, PNG, JPG) |

## Installation

```bash
pip install nairobi-os
```

Or build from source:
```bash
cargo build --release
# The canvas demo is a Rust binary - see examples/canvas_compile_demo.rs
```

## Usage

### Rust (Native)

Run the demo application:
```bash
cargo run --example canvas_compile_demo
```

### Python

Using the installed package:
```python
import nairobi_os as nb

# Open the visual canvas for DAG compilation
dag_bytes = nb.canvas.open()

# Execute the compiled pipeline
if dag_bytes:
    nb.canvas.execute(dag_bytes)
```

Or run the complete test script:
```bash
python test_canvas.py
```

This script performs:
1. `nairobi_os.ignite()` - Spawns Axum Refinery and Nairobi Hub daemons
2. `nb.canvas.open()` - Launches the visual node graph editor
3. `nb.canvas.execute(dag_bytes)` - Executes the compiled pipeline with timing metrics

The canvas exports a GVariant-encoded DAG that can be:
- Executed via `nb.canvas.execute()`
- Saved to disk for later use
- Transmitted over D-Bus/shared memory

## Building Graphs

1. **Right-click** on the canvas grid to open the node menu
2. Select a node type (Ingest, SQL Query, Axiom Crunch, or Lagos Plot)
3. **Connect** nodes by dragging from output pins (blue) to input pins (green)
4. Click **Compile Graph** to serialize the workflow

## Execution Flow

```
Canvas Graph → GVariant DAG → Nairobi Hub → Axum Refinery / Lagos Vision
```

The compiled DAG is transmitted via IPC to the Hub, which routes nodes to:
- **Axum Refinery**: Data ingestion and statistical processing
- **Lagos Vision**: Hardware-accelerated visualization rendering

## Implementation Notes

- Follow existing code documentation style (Apache 2.0 header, brief module description)
- Reference the exact node input/output counts in the table (critical for understanding the DAG structure)
- The canvas currently outputs to console - mention this for developers building from source
- Note that file picker uses `rfd` crate in dev-dependencies, not main dependencies
- Keep the README focused on usage; link to main repository README for architecture context