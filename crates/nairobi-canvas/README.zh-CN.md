[English](README.md) | [简体中文](README.zh-CN.md) | [廣東話](README.yue.md) | [Français](README.fr.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [Suomi](README.fi.md) | [Nederlands](README.nl.md) | [日本語](README.ja.md) | [Русский](README.ru.md) | [한국어](README.ko.md)

# Nairobi Canvas: 立即模式节点图视觉编译器

Nairobi Canvas 是一个硬件加速的视觉编译器，用于构建数据处理管道。它提供了一个基于 `egui`/`egui-snarl` 的立即模式节点图用户界面，可将视觉工作流编译为 GVariant DAG（有向无环图）格式，由 Nairobi Hub 执行。

## 功能

- **视觉管道构建器**：用于数据工作流的拖放式节点图界面
- **原生文件选择器**：点击 Ingest 节点上的 📂 按钮以浏览 CSV 文件
- **SQL 查询预设**：预先配置的查询模板（所有列、单列、Where 子句、多列）
- **GVariant 序列化**：将图编译为 GVariant 格式，以实现零拷贝 IPC
- **拓扑排序**：自动循环检测和执行排序

## 节点类型

| 节点 | 输入 | 输出 | 描述 |
|------|--------|---------|-------------|
| **Ingest** | 0 | 1 | 通过原生文件选择器加载 CSV 数据集 |
| **SqlQuery** | 1 | 1 | 对输入数据执行 Polars SQL 查询 |
| **AxiomCrunch** | 1 | 1 | 计算统计数据（平均值、标准差、峰度） |
| **LagosPlot** | 1 | 0 | 渲染视觉化（折线图、散点图、PNG、JPG） |

## 安装

```bash
pip install nairobi-os
```

或从源代码构建：
```bash
cargo build --release
# Canvas 演示是一个 Rust 二进制文件 - 请参阅 examples/canvas_compile_demo.rs
```

## 使用方法

### Rust (原生)

运行演示应用程序：
```bash
cargo run --example canvas_compile_demo
```

### Python

使用已安装的软件包：
```python
import nairobi_os as nb

# 打开用于 DAG 编译的视觉画布
dag_bytes = nb.canvas.open()

# 执行编译后的管道
if dag_bytes:
    nb.canvas.execute(dag_bytes)
```

或运行完整的测试脚本：
```bash
python test_canvas.py
```

此脚本执行：
1. `nairobi_os.ignite()` - 启动 Axum Refinery 和 Nairobi Hub 守护进程
2. `nb.canvas.open()` - 启动视觉化节点图编辑器
3. `nb.canvas.execute(dag_bytes)` - 执行带有计时指标的编译管道

画布导出一个 GVariant 编码的 DAG，可以：
- 通过 `nb.canvas.execute()` 执行
- 保存到磁盘以备后用
- 通过 D-Bus/共享内存传输

## 构建图

1. 在画布网格上**右键单击**以打开节点菜单
2. 选择节点类型（Ingest、SQL Query、Axiom Crunch 或 Lagos Plot）
3. 通过从输出引脚（蓝色）拖动到输入引脚（绿色）来**连接**节点
4. 单击 **Compile Graph** 来序列化工作流

## 执行流程

```
画布图表 → GVariant DAG → Nairobi Hub → Axum Refinery / Lagos Vision
```

编译后的 DAG 通过 IPC 传输到 Hub，Hub 将节点路由到：
- **Axum Refinery**：数据摄取和统计处理
- **Lagos Vision**：硬件加速的视觉化渲染

有关架构详情和完整的系统概述，请参阅[主存储库 README](../README.md)。

## 支持
如果您觉得 Nairobi OS 有用，请考虑支持该项目：

[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-PayPal-blue.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=chege.finance@gmail.com&item_name=Support+Nairobi+OS+Development)

## 许可证
本项目采用 **Apache License 2.0** 许可。

© 2026 Kevin Chege. 保留所有权利。
