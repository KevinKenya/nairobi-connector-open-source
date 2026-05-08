<!-- Author: Kevin Chege. Location: Nairobi -->

# Nairobi OS: Strategic Use Case Report

**Date**: May 8, 2026  
**Author**: Nairobi OS Project  
**Based on**: Benchmark Suite v2.0 — NBA Statistical Distillation (432MB, 837K rows)

---

## The Nairobi Proposition

Nairobi OS is not a general-purpose replacement for Pandas or Polars. It is a **purpose-built data analysis engine** designed for environments where **memory efficiency, CPU fairness, and predictable resource consumption** matter more than raw throughput.

This report identifies the specific deployment scenarios, workload types, and system constraints where Nairobi OS delivers clear, measurable advantage over alternatives.

---

## 1. Deployment Scenarios

### 1.1 🏗️ Memory-Constrained Environments

**The strongest use case.** Nairobi processes 432MB of CSV data in **20MB of RAM** — Pandas requires **4,285MB** for the same workload.

| Scenario | Why Nairobi Wins |
|----------|-----------------|
| **Edge devices** (Raspberry Pi, Jetson Nano, IoT gateways) | 512MB–2GB total RAM; Pandas would consume the entire system |
| **Containerized microservices** (Kubernetes pods with 64–256MB limits) | Nairobi fits inside a 64MB container; Pandas would OOM-kill |
| **Serverless functions** (AWS Lambda, GCP Cloud Functions) | Memory is billed per MB·ms; 209x less RAM = 209x lower memory cost |
| **Shared hosting / VPS** (1–4GB total RAM) | Multiple Nairobi instances can coexist; one Pandas job would starve the system |
| **Embedded analytics** (in-process analysis within a larger application) | The host application retains its memory budget |

> **Decision Rule**: If your environment has **< 1GB available RAM** for data processing, Nairobi is the correct choice. Pandas, Polars, and DuckDB all require 700MB–4.4GB for a 432MB dataset.

---

### 1.2 🖥️ Multi-Tenant / Shared-Resource Systems

Nairobi uses **10% CPU** vs Pandas' **1,158%**. This is 116x more CPU-efficient.

| Scenario | Why Nairobi Wins |
|----------|-----------------|
| **Multi-user analytics servers** | 10 concurrent Nairobi jobs = 100% CPU; 1 Pandas job = 1,158% CPU (starves other users) |
| **CI/CD pipelines** | Data validation in CI won't steal build cores |
| **Background data processing** | Analysis runs without impacting foreground user experience |
| **Fair-share scheduling** (cgroups, Kubernetes resource quotas) | Nairobi plays nicely within CPU quotas; NumPy-backed engines burst past them |

> **Decision Rule**: If you need to run **concurrent analytics jobs** on shared infrastructure without one job monopolizing all cores, Nairobi's single-digit CPU usage is the correct architecture.

---

### 1.3 📊 Ingestion-Dominated Workflows

Nairobi's ingestion is **2.3x faster** than Pandas (387ms vs 880ms) thanks to zero-copy `memfd` + kernel splice (`copy_file_range`).

| Scenario | Why Nairobi Wins |
|----------|-----------------|
| **ETL pipelines** where data loading is the bottleneck | Faster ingestion → faster pipeline completion |
| **Streaming CSV ingestion** (log files, sensor data) | Zero-copy architecture avoids memory pressure from repeated loads |
| **Large file scanning** (schema inspection, row counting) | Ingest + inspect without materializing the full DataFrame |
| **Data validation gates** (check schema before expensive processing) | Fast ingest → quick reject/accept decision |

> **Decision Rule**: If your pipeline spends **> 50% of its time loading data**, Nairobi's zero-copy ingestion delivers measurable speedup regardless of subsequent analysis complexity.

---

### 1.4 🔁 Predictable, Low-Jitter Workloads

Nairobi v2 has a **StdDev of 44ms** — Pandas has **86ms**, DuckDB has **638ms**. Nairobi is the most consistent engine.

| Scenario | Why Nairobi Wins |
|----------|-----------------|
| **SLA-bound analytics** (must complete within P99 latency target) | Nairobi's tight distribution means fewer SLA breaches |
| **Real-time dashboards** (refresh every N seconds) | Consistent latency → smooth UX; no random spikes |
| **Automated alerting pipelines** | Predictable execution time → reliable alert delivery |
| **Benchmarking and profiling** | Low jitter means fewer iterations needed for statistical significance |

> **Decision Rule**: If you need **< 100ms latency variance** between runs, Nairobi's deterministic performance profile is the best fit.

---

## 2. Workload Types

### 2.1 ✅ Ideal Workloads

| Workload | Nairobi Advantage | Example |
|----------|-------------------|---------|
| **Descriptive statistics** (mean, std, skew, kurtosis) | Competitive speed at 209x less RAM | Summarizing sensor data, financial returns, player stats |
| **Correlation analysis** (Pearson, Spearman) | Fused pipeline delivers both analytics + correlation in one call | Portfolio correlation, feature selection, variable dependency analysis |
| **Anomaly detection** (z-score > 3σ) | Built-in to the crunch pipeline, zero extra cost | Fraud detection, outlier flagging, quality control |
| **Schema inspection** | Zero-copy ingest → inspect without full parse | Data lake exploration, automated data cataloging |
| **Data cleaning** (null handling, column drops) | In-daemon processing, returns cleaned memfd | ETL preprocessing stages |
| **SQL queries on CSV** | Built-in Polars SQL engine within the Refinery | Ad-hoc analysis on flat files without database setup |

### 2.2 ⚠️ Marginal Workloads

| Workload | Trade-off | When to Use Nairobi Anyway |
|----------|-----------|---------------------------|
| **Complex aggregations** (group-by, pivot, window functions) | Polars/DuckDB are faster at SQL-heavy workloads | When memory constraints prohibit alternatives |
| **Iterative ML feature engineering** | Pandas' DataFrame API is more ergonomic for exploration | When the notebook server has < 2GB RAM |
| **Time-series resampling** | Specialized libraries (e.g., `polars.rolling`) are more optimized | When running on edge devices |

### 2.3 ❌ Not Recommended

| Workload | Why Not | Better Alternative |
|----------|---------|-------------------|
| **Interactive DataFrame exploration** (`.head()`, `.describe()`, slicing) | No DataFrame API — Nairobi returns JSON results, not DataFrames | Pandas, Polars |
| **GPU-accelerated ML** | No GPU pathway | cuDF, Rapids |
| **Sub-10ms latency requirements** | D-Bus IPC has an irreducible floor of ~300ms per call | In-process NumPy/Polars |
| **Datasets < 10MB** | Overhead of D-Bus + Refinery daemon exceeds the data processing time | Pandas (loads in < 50ms) |
| **Join-heavy workflows** (multi-table relational queries) | Single-table architecture | DuckDB, PostgreSQL |

---

## 3. Architecture Patterns

### 3.1 The Sidecar Pattern (Recommended)

```
┌──────────────────┐     D-Bus (memfd)     ┌────────────────────┐
│  Your Application │◄────────────────────►│  Nairobi Refinery   │
│  (Python / Rust)  │    20MB footprint     │  (Axum Daemon)      │
└──────────────────┘                        └────────────────────┘
```

Deploy the Refinery daemon as a **sidecar process** alongside your application. The application sends file paths over D-Bus; the Refinery returns analytics as GVariant payloads. Zero data copying between processes thanks to `memfd`.

**Best for**: Microservice architectures, Kubernetes pods, systemd-managed services.

### 3.2 The Embedded Pattern

```python
import nairobi_os

nairobi_os.start_refinery()  # Starts daemon, waits for D-Bus registration
result = nairobi_os.data.pipeline("data.csv", "price", "price,volume")
nairobi_os.stop_refinery()   # Clean shutdown
```

The Python package manages the Refinery lifecycle automatically. No external daemon management needed.

**Best for**: Scripts, notebooks, CI/CD pipelines, one-shot analysis jobs.

### 3.3 The Shared Daemon Pattern

```
┌── App A ──┐
│  ingest() │──┐
└───────────┘  │     ┌────────────────────┐
               ├────►│  Nairobi Refinery   │
┌── App B ──┐  │     │  (shared daemon)    │
│ pipeline()│──┘     └────────────────────┘
└───────────┘
```

Multiple applications share a single Refinery daemon. Each gets its own `memfd` handle — full process isolation with shared compute infrastructure.

**Best for**: Multi-tenant servers, shared analytics backends, platform-as-a-service.

---

## 4. Decision Matrix

Use this matrix to determine if Nairobi is the right choice for your specific situation:

| Factor | Nairobi ✅ | Pandas/Polars ✅ |
|--------|-----------|-----------------|
| Available RAM | < 1 GB | > 4 GB |
| Concurrent users | > 1 | 1 (dedicated) |
| Dataset size | 10MB – 1GB | Any |
| Primary metric | Memory/CPU efficiency | Raw speed |
| Latency requirement | > 500ms acceptable | < 100ms required |
| Workload type | Stats, correlation, anomaly | Exploration, ML, joins |
| Deployment | Containers, edge, serverless | Notebooks, workstations |
| Budget constraint | Memory-billed (serverless) | Compute-billed (GPU) |

---

## 5. Cost Analysis (Serverless Example)

For a serverless function processing 1,000 CSV files per hour on AWS Lambda:

| Engine | Memory Config | Cost per Invocation | Monthly Cost (720K invocations) |
|--------|--------------|--------------------|---------------------------------|
| **Pandas** | 4,096 MB | $0.000067 × 1.0s = $0.000067 | **$48.24** |
| **Nairobi** | 128 MB | $0.0000021 × 1.2s = $0.0000025 | **$1.80** |

**Nairobi is 27x cheaper** for serverless analytics workloads, even though each invocation takes 20% longer. The memory savings dominate the cost equation.

> The 209x memory advantage translates directly to infrastructure cost savings in memory-billed environments.

---

## 6. Performance Envelope

Based on benchmarked results (432MB CSV, 837K rows, AMD Ryzen 4650U):

```
                    ┌─────────────────────────────────────────┐
   Faster ──►       │                                         │
                    │  Polars ●           (275ms, 761MB)      │
                    │                                         │
                    │                                         │
                    │  Pandas ●           (912ms, 4285MB)     │
                    │                                         │
                    │  Nairobi v2 ●       (1160ms, 20MB)      │
                    │                                         │
                    │                                         │
                    │                                         │
                    │                                         │
                    │  DuckDB ●           (2904ms, 917MB)     │
                    │                                         │
   Slower ──►       └─────────────────────────────────────────┘
                    Less RAM ◄──────────────────────► More RAM
                    (20MB)                            (4285MB)
```

Nairobi occupies the **bottom-left corner** — moderate speed, minimal resources. This is the correct trade-off for resource-constrained deployments.

---

## 7. Recommendations Summary

### Use Nairobi OS when:
1. **Memory is scarce** — edge, containers, serverless, embedded
2. **CPU must be shared** — multi-tenant, background processing, CI/CD
3. **Ingestion speed matters** — ETL pipelines, streaming data
4. **Consistency is critical** — SLA-bound analytics, real-time dashboards
5. **Cost optimization** — serverless billing, cloud resource quotas

### Do not use Nairobi OS when:
1. **Sub-100ms latency is required** — use in-process NumPy/Polars
2. **Interactive exploration** — use Pandas in a notebook
3. **Dataset is < 10MB** — D-Bus overhead exceeds processing time
4. **Complex multi-table joins** — use DuckDB or a proper database

### The sweet spot:
> **10MB–1GB CSV files**, **descriptive statistics + correlation**, **in a memory-constrained environment** where you need **predictable, efficient execution** without monopolizing system resources.

---

**Report Generated**: May 8, 2026  
**Data Source**: Nairobi Benchmarks v2.0  
**Contact**: Kevin Chege @ Nairobi OS Project
