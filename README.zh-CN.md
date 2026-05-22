[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md)

# Nairobi OS

## 概述
Nairobi OS 是一个高性能、分布式的 AI 和数据科学基础设施，专为极端的资源效率而设计。它利用专门的基于 Rust 的 refinery 守护程序，支持在受限环境（Edge、IoT、Serverless）中处理海量数据集，并通过其兼容 MCP 的无障碍桥接器提供**“无像素计算机使用”**。

通过利用 `io_uring`、`memfd` 和大页 (Huge Pages) 等内核级特性，Nairobi OS 实现了亚毫秒级的 IPC 延迟和零拷贝数据管道。

## 演示视频

https://github.com/user-attachments/assets/demo.webm

<video src="assets/demo.webm" controls width="100%">
  您的浏览器不支持视频标签。
</video>

## 主要特性
- **无像素计算机使用**：直接通过 AT-SPI2 和 TOON（面向 Token 的对象表示法）压缩算法与 Linux 桌面交互，从而跳过对 AI 代理的 OCR 或视觉处理。
- **零拷贝注入**：使用 `io_uring` 和 1GB 大页实现硬件加速的数据加载。
- **硬件加速可视化**：通过 Lagos Vision 引擎（`wgpu` 和 `egui`）进行交互式 Jupyter 绘图。
- **融合分析管道**：在一次 D-Bus 往返中提取、处理和关联数据。
- **主权接口 (Sovereign Interface)**：一个流畅的 Python API，隐藏了低级 IPC 和内存管理的复杂性。

## 架构
1.  **Nairobi Axum Refinery**：高性能 Rust 核心。
2.  **Nairobi Hub**：IPC 协调器。
3.  **Lagos Vision**：视觉皮层。无头渲染引擎。
4.  **Nairobi Connector**：语义桥接器。向 LLM 暴露 Linux 桌面无障碍树的 MCP 服务器。
5.  **Nairobi Python**：高级桥接器。提供 Pythonic 接口。

## 安装

### 通过 PyPI
```bash
pip install nairobi-os
```

### 从源码构建
```bash
git clone https://github.com/KevinKenya/nairobi-connector-open-source
cd nairobi-connector-open-source
python3 -m venv .venv
source .venv/bin/activate
pip install maturin pyo3-build-config zbus anywidget traitlets
./build_wheel.sh
```

## 使用方法

### 数据分析
```python
import nairobi_os

nairobi_os.connect()
df = nairobi_os.read_csv("dataset.csv")
print(f"平均值: {df.column_name.mean()}")
df.plot()
```

### 计算机使用 (MCP 服务器)
使用 Nairobi Connector 的 AI 代理应遵循此基本循环：
1. 使用 `nairobi_find_window` 定位窗口。
2. 通过 `nairobi_get_ui_map` 观察当前状态。
3. 读取所需元素的 TOON `[ID: N]`。
4. 通过 `nairobi_interact` 或 `nairobi_type_text` 执行操作。

## 支持
如果您觉得 Nairobi OS 对您有所帮助，请考虑支持该项目：

[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-PayPal-blue.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=chege.finance@gmail.com&item_name=Support+Nairobi+OS+Development)

## 许可证
本项目采用 **Apache License 2.0** 许可证。
*(注：TOON 格式的部分内容归 TOON 作者所有。)*

---
© 2026 Kevin Chege。保留所有权利。
