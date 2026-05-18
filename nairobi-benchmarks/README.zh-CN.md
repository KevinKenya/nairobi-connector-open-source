[English](README.md) | [简体中文](README.zh-CN.md)

# Nairobi 基准测试套件 (Nairobi Benchmark Suite)

## 概览
Nairobi 基准测试套件是一个严格的性能评估框架，旨在将 Nairobi OS 与行业标准的数据处理库（如 Pandas）进行比较。它专注于端到端延迟、内存效率以及“融合分析打击 (Fused Analytical Strikes)”对实际工作负载的影响。

## 核心指标
- **摄取延迟 (Ingestion Latency)**：将数据从磁盘加载到内存驻留结构中的时间。
- **计算密度 (Compute Density)**：在繁重分析负载期间的峰值常驻内存大小 (RSS)。
- **流水线吞吐量 (Pipeline Throughput)**：融合摄取-处理-关联操作的总时间。

## 安装

### 前置条件
- Python 3.10+
- Nairobi OS（已安装并配置）

### 设置
```bash
cd nairobi-benchmarks
pip install -r requirements.txt
```

## 运行基准测试

### 1. 准备数据集
生成合成数据集以测试扩展性：
```bash
# 生成千万行数据集
python datasets/generators/generate_synthetic.py --type tall --output datasets/synthetic/tall_10m.csv
```

### 2. 运行工作负载
执行特定的基准测试工作负载：
```bash
python orchestration/benchmark_runner.py --workload workloads/workload_statistical_distillation.yaml --iterations 10
```

### 3. 分析结果
基准测试结果以 JSON 格式存储，可以使用包含的绘图工具进行可视化：
```bash
python visualization/plot_scaling.py
```

## 方法论
该套件遵循“硬件优先”的基准测试方法，确保：
- 分别测量冷启动和热启动。
- 在运行之间尽可能清除内核缓存。
- 所有计算都使用 `result_validator.py` 进行数学恒等验证 (±1e-5)。

## 许可证
该套件是 Nairobi OS 项目的一部分，根据 **PolyForm Noncommercial License 1.0.0** 获得许可。

---
© 2026 Kevin Chege. 保留所有权利。
