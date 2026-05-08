<!-- Author: Kevin Chege. Location: Nairobi -->

# nairobi-benchmarks/methodology.md
# Nairobi OS Performance Methodology

## 1. Objective
This suite is designed to evaluate the performance of Nairobi OS against industry-standard data processing engines (Pandas, Polars, DuckDB) under rigorous scientific conditions.

## 2. Fairness Principles
*   **Idiomatic Code**: All competitors utilize their most optimized, idiomatic execution paths.
    *   **Pandas**: Uses vectorized operations and the PyArrow backend for ingestion.
    *   **Polars**: Uses the **Lazy API** (`.lazy()...collect()`) to allow query optimization.
    *   **DuckDB**: Uses standard SQL with direct CSV scanning.
*   **Standardized Output**: Every engine must return identical mathematical results (within a `±1e-5` tolerance). If an engine fails the math, its latency results are disqualified.

## 3. Cache Methodology
To isolate the effects of the operating system's page cache and the engine's internal caching:
*   **Cold Cache**: Before each engine session, a system-level cache drop is attempted: `sync; echo 3 > /proc/sys/vm/drop_caches`.
*   **Isolation**: Engines are run in separate sessions to prevent cross-contamination of memory or CPU state.

## 4. Metrics Collection
*   **Latency**: Measured as Wall Time using `time.perf_counter_ns` for nanosecond precision.
*   **Peak RAM**: Captured using a combination of `psutil` (background polling) and `resource.getrusage` (Peak RSS).
*   **Peak CPU**: Tracked via `psutil` background thread to measure engine-level parallelization efficiency.

## 5. Execution Policy
*   **Iterations**: Minimum 10 runs per workload per engine.
*   **Reporting**: We report the Mean and Standard Deviation of latency to ensure statistical significance.
*   **Hardware Setup**:
    *   **CPU**: [Placeholder: e.g., AMD Ryzen 4650U]
    *   **RAM**: [Placeholder: e.g., 16GB]
    *   **OS**: [Placeholder: e.g., Ubuntu 22.04 LTS]
    *   **Disk**: [Placeholder: e.g., NVMe SSD]

## 6. Workloads
1.  **Statistical Distillation**: High-order moments (Skewness/Kurtosis) on 10M rows.
2.  **Wide Dataset**: Cache locality and memory footprint testing with 1,000 columns.
3.  **Relational Strike**: Streaming Pearson correlation between massive columns.
