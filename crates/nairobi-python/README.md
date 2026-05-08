# Nairobi OS: Heavy Iron AI Infrastructure

**Author**: Kevin Chege. Location: Nairobi  
**License**: PolyForm Noncommercial License 1.0.0

Nairobi OS is a high-performance data science operating system primitive designed for extreme resource efficiency. It enables memory-constrained environments (Edge, Containers, Serverless) to process large-scale datasets with **200x less RAM** than traditional Python data stacks.

## 🚀 Version 0.2.0: The Fused Strike
This release introduces the **Fused Analytics Pipeline**, allowing ingestion, statistical distillation, and correlation to happen in a single, high-speed D-Bus round trip.

### Key Features:
- **Fused Pipeline**: `nairobi_os.data.pipeline()` for maximum throughput.
- **Zero-Copy Ingestion**: Powered by `memfd` and kernel `copy_file_range`.
- **Extreme Efficiency**: Process 400MB+ datasets in just 20MB of RAM.
- **Rayon Parallelization**: Vectorized analytics leveraging multi-core hardware.
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

## 📊 Performance Benchmark (v2.0)
| Metric | Pandas | Nairobi OS |
|--------|--------|------------|
| RAM Usage | 4,285 MB | **20.5 MB** |
| CPU Usage | 1,158% | **10%** |
| Latency | 912 ms | 1,160 ms |

## ⚖️ Licensing
This project is licensed under the **PolyForm Noncommercial License 1.0.0**. It is free for personal, educational, and research use. For commercial inquiries, please contact the author.

---
© 2026 Kevin Chege. All Rights Reserved.
