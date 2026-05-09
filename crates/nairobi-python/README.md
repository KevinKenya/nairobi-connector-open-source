# Nairobi OS: Heavy Iron AI Infrastructure

> [!IMPORTANT]
> **Linux & WSL2 Only.** We do not compromise on kernel physics.

**Author**: Kevin Chege. Location: Nairobi  
**License**: PolyForm Noncommercial License 1.0.0

Nairobi OS is a high-performance data science operating system primitive designed for extreme resource efficiency. It enables memory-constrained environments (Edge, Containers, Serverless) to process large-scale datasets with **hardware-accelerated ingestion** and vectorized Rust analytics.

## 🚀 Version 0.2.1: The Fused Strike
This release introduces the **Fused Analytics Pipeline**, allowing ingestion, statistical distillation, and correlation to happen in a single, high-speed D-Bus round trip.

### Key Features:
- **Fused Pipeline**: `nairobi_os.data.pipeline()` for maximum throughput.
- **Zero-Copy Ingestion**: Powered by `memfd`, `io_uring`, and kernel `copy_file_range`.
- **Extreme Speed**: Ingest 450MB+ datasets **12x faster** than standard Pandas.
- **Rayon Parallelization**: Vectorized analytics leveraging multi-core hardware saturation.
- **Persistent Infrastructure**: Cached D-Bus connections for low-latency calls.

## 🛠️ Installation
```bash
pip install nairobi-os
```

## 💻 Quick Start
```python
import nairobi_os
import json

# Ignite the refinery daemon
nairobi_os.start_refinery()

# Run a fused analytics strike
# (Ingest + Mean/StdDev/Skew/Kurtosis + Pearson Correlation)
result_json = nairobi_os.data.pipeline(
    "data.csv", 
    "target_column", 
    "col1,col2" # Correlation pair
)

result = json.loads(result_json)
print(f"Mean: {result['mean']}")
print(f"Correlation: {result['pearson']}")

# Shutdown refinery
nairobi_os.stop_refinery()
```

## 📊 Performance Benchmark (v0.2.1)
| Metric | Pandas (Unoptimized) | Nairobi OS | Speedup |
|--------|--------|------------|---------|
| **Ingestion Latency** | 6.38s | **0.52s** | **12.2x** |
| **Statistical Distillation** | 8.79s | **1.59s** | **5.5x** |
| **Total Pipeline** | 6.42s | **2.29s** | **2.8x** |

## ⚖️ Licensing
This project is licensed under the **PolyForm Noncommercial License 1.0.0**. It is free for personal, educational, and research use. For commercial inquiries, please contact the author.

---
© 2026 Kevin Chege. All Rights Reserved.
