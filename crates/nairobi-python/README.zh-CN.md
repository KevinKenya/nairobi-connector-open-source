[English](README.md) | [简体中文](README.zh-CN.md)

# Nairobi Python

## 概览
Nairobi Python 提供了连接 Nairobi OS 基础设施的高级桥梁。它使数据科学家能够通过熟悉的 Python 化接口，利用基于 Rust 的硬件加速分析能力。该软件包处理守护进程管理、IPC 协调和内存映射，使用户能够专注于数据分析。

## 核心特性
- **SovereignFrame**：一个流畅的、类似 Pandas 的接口，用于管理远程数据句柄。
- **延迟启动 (Lazy Ignition)**：在首次访问数据时自动启动并配置 refinery 守护进程。
- **Jupyter 集成**：对使用 Lagos Vision 组件进行交互式可视化提供一流支持。
- **零拷贝桥梁 (Zero-Copy Bridge)**：直接消费来自 Rust refinery 的 `memfd` 句柄，开销控制在亚毫秒级。

## 安装

### 从 PyPI 安装
```bash
pip install nairobi-os
```

### 从源码安装
```bash
cd crates/nairobi-python
pip install -e .
```
*注意：从源码构建需要安装 Rust 工具链和 `maturin`。*

## 使用

### 快速开始
```python
import nairobi_os

# 连接到 refinery（自动处理 D-Bus 和守护进程启动）
nairobi_os.connect()

# 摄取 CSV 文件
df = nairobi_os.read_csv("data.csv")

# 使用流畅 API 获取统计信息
mean_val = df.column_name.mean()
p99_val = df.column_name.p99()

# 直接在引擎上运行 SQL 查询
tall_players = df.query("SELECT * FROM dataset WHERE height > 80")

# 使用 Lagos Vision 绘图
tall_players.plot()
```

## API 参考

### `nairobi_os.connect()`
初始化环境，必要时启动 D-Bus 会话，并点燃 refinery 守护进程。

### `nairobi_os.read_csv(path, delimiter=",", encoding="utf-8")`
使用 refinery 的零拷贝流水线摄取 CSV 文件。返回一个 `SovereignFrame`。

### `SovereignFrame` 方法
- `df.column.mean()`：计算算术平均值。
- `df.column.std_dev()`：计算标准差。
- `df.column.p95()`, `df.column.p99()`：计算百分位数。
- `df.column.skewness()`, `df.column.kurtosis()`：计算统计矩。
- `df.query(sql_string)`：在数据集上执行 Polars-SQL。
- `df.correlate("col1,col2")`：计算 Pearson 和 Spearman 相关系数。
- `df.plot(width, height)`：显示交互式 `anywidget` 可视化。

## 开发

### 添加新的 Python 绑定
Nairobi Python 使用 PyO3 与 Rust 接口。核心功能应添加到 `crates/nairobi-python/src/lib.rs` 并通过 `nairobi_os._core.data` 模块公开。

### 测试
Python 包的集成测试可以使用 `pytest`（如果已配置）或提供的测试脚本运行：
```bash
python3 test_nairobi.py
```

要在没有完整 refinery 的情况下进行隔离测试，你可以模拟 `_core.data` 模块或使用带有现有句柄的 `SovereignFrame`。

## 常见问题排查
- **Refinery 未能在 D-Bus 上注册**：这通常发生在无头环境中。确保 `dbus-launch` 可用，或调用 `nairobi_os.connect()`，它会尝试修复环境。
- **句柄未找到 (Handle Not Found)**：数据句柄是与会话绑定的。如果 refinery 重启，之前的 `SovereignFrame` 句柄将失效。

## 许可证
该项目根据 **PolyForm Noncommercial License 1.0.0** 获得许可。

---
© 2026 Kevin Chege. 保留所有权利。
