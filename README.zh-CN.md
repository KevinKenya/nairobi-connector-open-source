# Nairobi OS: 高性能、零拷贝 AI 与数据科学基础设施

[![PyPI Version](https://img.shields.io/pypi/v/nairobi-os.svg)](https://pypi.org/project/nairobi-os/)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)
![System](https://img.shields.io/badge/Kernel-Linux_6.17_Native-orange.svg)
![Arch](https://img.shields.io/badge/Architecture-x86__64_/_ARM64-red.svg)

---

## 起源：从熔炉到金属

Nairobi OS 并非诞生于舒适的企业孵化器或风投支持的研究实验室。它是绝对必然的产物，源于一系列深刻的个人危机，以及在标准行业工具失效时坚持执行的无情驱动力。

我是 Kevin Chege，Sovereign Systems Lab（肯尼亚内罗毕）的创始人。从 2009 年到 2022 年，我的生活被严重的酗酒所吞噬。它让我付出了职业声誉、机遇以及几乎生命的代价。在网瘾/酒瘾最严重的时期，我曾担任英国米尔顿凯恩斯开放大学战略办公室的分析师，此前曾担任 AIESEC 卢旺达创始人兼总裁（2006-2010年）。今天，是我持续清醒的第四年。

```
                     LEGIO XIII GEMINA
              "第十三军团 — 六月十三日"
     十三年失去。十三年去夺回。
```

我的编程之路植根于低级系统架构和极端优化。2015 年，我在[这篇关于肯尼亚硅谷的论文](https://www.linkedin.com/pulse/building-kenyas-silicon-valley-making-work-kevin-chege/)中阐述了在非洲大陆建立去中心化、高度技术性能力的愿景。当 2023 年 LLM 淘金热开始时，我占得先机。我构建并部署了 LLM 封装器（wrappers），但很快就意识到了它们的局限性，正如这份早期的 [2023 年 LLM 封装器演示](https://www.linkedin.com/feed/update/urn:li:activity:7102930955807449088/)所记录的那样。

我意识到，在不稳定的 API 之上构建高级封装器是架构上的死胡同。真正的战争是在本地硬件限制和资源分配的交汇处进行的。

在整个 2025 年期间，我一直生活在一台硬件配置高度受限的 Lenovo X13 ThinkPad 上：

```
处理器: AMD Ryzen 5 PRO 4650U (6 核, 12 线程)
显卡: AMD Radeon RX Vega 6 集显 (iGPU)
内存: 32 GB RAM (系统占用率极高)
存储: 256 GB NVMe (99% 已满)
```

就在这台机器上，我在 2025 年开发了 **Tumz** ([Sarafakai](http://www.sarafakai.com))，这是一款气隙隔离（air-gapped）、零延迟的临床决策支持 AI。它在集显（iGPU）上同时执行实时语音转录和临床推理，将整个统一医学语言系统（UMLS）常驻于内存中。我们目前正与肯尼亚的一家医院合作，试用 Tumz 进行为期一年的临床试验——因为人类健康需要严格的实证检验，而不是开发者的臆测。

在开发 Tumz 的过程中，我遇到了现代数据科学技术栈巨大且系统性的低效问题：
1. **Python 税**：端到端内存复制、GIL 瓶颈以及巨大的运行时开销。
2. **浏览器税**：Manifest V3 的复杂性、渲染延迟以及长时间运行的智能体（agent）对话中的高频通信故障。
3. **OS 内核瓶颈**：低效的进程调度、CPU 线程饥饿以及显示服务器开销（Wayland 与 X11 上下文切换）。

因此，在 2025 年底，我着手构建一个完全绕过这些限制的底层基础架构栈——一个专为零拷贝数据管道和硬件原生 AI 执行而设计的智能体操作系统（Agentic Operating System）。本仓库正是该引擎的开源核心。

---

## 全球采用与遥测

Nairobi OS 于 2026 年 5 月 6 日发布，迅速获得了全球系统程序员、量化研究员和边缘计算架构师的青睐。

### 全球累计分布 (2026年5月6日 – 2026年5月23日)

| 指标 | 测量值 | 背景信息 |
| :--- | :--- | :--- |
| **全球排名** | **#75,293** | 在 PyPI 上 797,894 个活跃包中 |
| **百分位** | **9.43%** | 系统级 Python 扩展的顶级排名 |
| **总下载量** | **1,525** | 干净、有机、高意向的开发者下载 |

### 各版本下载量

```
  0.2.0 [████████████████████████████████████████] 342
  0.2.1 [██████████████████████████] 224
  0.3.0 [████████████████████████] 212
  0.3.1 [████████████████████] 176
  0.1.0 [███████████████████] 169
  0.4.1 [██████████████] 120
```

### 排名前 10 的采用主权地区

| 排名 | 地区 | 国家代码 | 下载量 |
| :--- | :--- | :--- | :--- |
| 1 | 美国 | US | 661 |
| 2 | 中国香港 | HK | 103 |
| 3 | 中国大陆 | CN | 84 |
| 4 | 德国 | DE | 74 |
| 5 | 日本 | JP | 65 |
| 6 | 新加坡 | SG | 56 |
| 7 | 英国 | GB | 51 |
| 8 | 法国 | FR | 51 |
| 9 | 俄罗斯 | RU | 42 |
| 10 | 韩国 | KR | 30 |

---

## 支持与主权

如果 Nairobi OS 优化了您的数据管道、降低了您的云服务账单，或驱动了您的本地智能体架构，请考虑支持我们的独立系统研究。每一笔贡献都将直接用于内罗毕的硬件级编译器优化和边缘计算测试。

[![支持 Nairobi OS 开发](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-PayPal-blue.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=chege.finance@gmail.com&item_name=Support+Nairobi+OS+Development)

---

## 语言选项

[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [Русский](README.ru.md) | [한국어](README.ko.md)

---

## 核心特性

* **无像素计算机使用**：绕过缓慢、昂贵的基于视觉的智能体管道。通过 AT-SPI2 和 TOON（面向 Token 的对象表示法）压缩算法与 Linux 桌面原生交互，直接将原始层次树馈送给 LLM。
* **零拷贝数据接入**：利用 `io_uring` 和 1GB 大页（Huge Pages）实现硬件加速、内核旁路的快速数据加载。
* **硬件加速可视化**：基于 `wgpu` 和 `egui` 构建的 `lagos-lite` 渲染守护进程，提供低延迟、交互式的 Jupyter 绘图。
* **向量化分析执行**：利用 Polars 查询执行和 Rayon 多线程数据管道实现极端的 CPU 饱和度。
* **主权接口**：一个流畅的 Python API (`SovereignFrame`)，封装了内存映射文件描述符和进程间通信（IPC）。

---

## 开源与企业级架构

Nairobi OS 在结构上是分叉的。开源仓库提供基础的高性能数据处理和单节点可视化基元。闭源的企业级生态系统则包含先进的多智能体、高可用性以及特定行业的实现。

```
                                  +---------------------------------------+
                                  |         Nairobi Python API            |
                                  +---------------------------------------+
                                                      |
                                     [ 通过 D-Bus / 共享内存传输 GVariant ]
                                                      |
                                                      v
                                  +---------------------------------------+
                                  |           Nairobi Hub                 |
                                  +---------------------------------------+
                                                      |
                    +---------------------------------+---------------------------------+
                    |                                                                   |
                    v                                                                   v
     +------------------------------+                                    +------------------------------+
     |     Axum Refinery (数据)     | <===[ 零拷贝 IPC / iceoryx2 ]===> |     Lagos Vision (视觉)      |
     +------------------------------+                                    +------------------------------+
```

### 开源 Crate 工作区 (`crates/`)

1. **`nairobi-axum-refinery`**：高性能 Rust 守护进程，管理原始数据接入、Rayon 并行化统计计算以及 Polars 向量化查询执行。
2. **`nairobi-hub`**：中央 IPC 协调器。管理并路由客户端与 refinery 守护进程之间的文件描述符和信号。
3. **`lagos-lite`**：视觉皮层。一个无头、事件驱动的渲染引擎，将内存映射文件直接映射到 GPU 管道中。
4. **`nairobi-protocol`**：共享协议层。定义标准的 GVariant 序列化方案、错误类型和共享内存布局。
5. **`nairobi-python`**：通过 `PyO3` 编译并使用 `Maturin` 打包的 Python 扩展模块。

### 私有企业生态系统 (`modules/`)

我们的企业级组件保存在私有仓库（`Sovereign-Systems-Lab`）中，面向工业、金融和国家级基础设施进行授权。

1. **`sovereign-ui`**：企业级 AT-SPI2 引擎。实现 Aegis 协议安全、硬件绑定以及生产级桌面操作。
2. **`nairobi-connector`**：高级模型上下文协议（MCP）服务器，管理企业 LLM 的原始、低延迟 D-Bus 信号。
3. **`tactical-rtos-node`**：用于安全关键型边缘工业自动化的超低延迟、实时操作系统调度器。
4. **`industrial-guardian-rust` / `industrial-guardian-python`**：自主站点可靠性工程（SRE）层，具有预测性 OOM、内存泄漏和系统崩溃规避功能。
5. **`fintech-bridge-rust`**：实时高频交易解析器和传统主机桥接器（EBCDIC/SBA 终端解析）。
6. **`aviation-audio-rust`**：亚毫秒级、无锁音频流处理、声学遥测分析和原始波形 DSP。
7. **`drawbridge_api`**：安全、经过身份验证的多租户 gRPC 吊桥，将本地内核与不受信任的云智能体调用隔离。

### 能力对比矩阵

| 能力 / 特性 | 开源核心 (`crates/`) | 企业套件 (`modules/`) |
| :--- | :---: | :---: |
| **接入引擎** | `mmap` / `copy_file_range` | `io_uring` + `SQPOLL` + 1GB 大页 |
| **统计分析** | 基础描述性统计 | 向量化、多通道偏度/峰度、相关性分析 |
| **查询引擎** | 进程内 Polars SQL | 分布式 Apache Arrow / DataFusion 集群 |
| **IPC 机制** | POSIX 共享内存 / D-Bus | 零拷贝 `iceoryx2` 共享内存域 |
| **可视化** | 本地 Jupyter `anywidget` | WebRTC GStreamer / 透明 Wayland Layer-Shell 覆盖 |
| **安全与合规** | 标准 POSIX 边界 | Aegis 协议，SHA-256 链式取证账本 |
| **身份验证** | 无（本地受信任用户） | 硬件绑定（TPM 2.0 / CPU ID），私有 PKI |
| **平台目标** | 单节点 Linux | 分布式云 / 边缘节点 / 高频交易 |

---

## 安装与设置

### 系统要求
- **操作系统**：Linux（推荐 Ubuntu 22.04+）或 Windows Subsystem for Linux (WSL2)。
- **显卡**：兼容 Vulkan、Metal 或 OpenGL 的驱动程序。
- **Python**：3.10 或更高版本。
- **Rust**：稳定的工具链（如果从源码构建）。

### 快速安装 (PyPI)
```bash
pip install nairobi-os
```

### 从源码构建
要编译整个工作区，包括原生守护进程和 Python 扩展：

1. **克隆仓库**：
   ```bash
   git clone https://github.com/KevinKenya/nairobi-connector-open-source.git
   cd nairobi-connector-open-source
   ```

2. **配置虚拟环境**：
   ```bash
   python3 -m venv .venv
   source .venv/bin/activate
   pip install maturin pyo3-build-config zbus anywidget traitlets pandas
   ```

3. **执行工作区构建**：
   ```bash
   chmod +x build_wheel.sh
   ./build_wheel.sh --release
   ```
   这将编译原生守护进程，将其复制到包目录，并在 `crates/nairobi-python/target/wheels/` 下构建 wheel 包。

---

## 使用指南

### 1. 数据分析 (内存管道)

Nairobi OS 提供了 `SovereignFrame` API。它在底层处理原始内存映射，从而实现快速的数据操作。

```python
import nairobi_os as nb

# 启动背景 refinery 守护进程
nb.connect()

# 使用零拷贝内存管道接入数据集
frame = nb.read_csv("simulator/fndds_ingredient_nutrient_value.csv")

# 通过 Rust refinery 执行向量化计算
profile = frame.crunch("value")
print(f"均值: {profile['mean']:.4f}")
print(f"标准差: {profile['std_dev']:.4f}")

# 直接在内存映射的 frame 上执行任意 SQL 查询
subset = frame.query("SELECT * FROM dataset WHERE value > 50.0")

# 唤起 Lagos 加速的交互式绘图小部件
subset.plot(column="value")
```

### 2. 无像素计算机使用 (MCP)

要使用 AT-SPI2 语义接口，您的 AI 智能体应与公开的 MCP 服务器工具进行交互，而不是读取屏幕截图：

```
                     计算机使用序列
                     
  [ LLM 智能体 ]                                [ Nairobi OS ]
        |                                             |
        |===> nairobi_find_window("Text Editor") ====>| (定位目标)
        |<=== 返回窗口 ID 和边界 =====================|
        |                                             |
        |===> nairobi_get_ui_map() ==================>| (生成 TOON)
        |<=== 返回压缩的 Markdown 树 =================|
        |     "[ID: 12] Button: 'Save'"               |
        |                                             |
        |===> nairobi_interact(12, "click") =========>| (执行操作)
        |<=== 返回成功状态 ===========================|
```

---

## 系统微调 (贡献者指南)

为了达到我们基准测试中所示的性能配置文件，您的主机内核必须配置为系统级内存映射。

### 1GB 大页 (Huge Pages)
Nairobi OS 使用 1GB 大页来绕过 CPU 在海量数据集上的转换旁路缓冲区（TLB）转换开销。

要在 Linux 主机上分配一个大页：
```bash
echo 1 | sudo tee /sys/kernel/mm/hugepages/hugepages-1048576kB/nr_hugepages
```
*注意：如果系统由于碎片化而无法分配 1GB 页面，引擎会自动回退到透明大页 (THP)。*

### D-Bus Broker 配置
在高频环境中，请确保安装了 `dbus-broker` 而不是传统的 `dbus-daemon`，以便在控制平面上实现快速的信号传播。

---

## 许可证

本项目采用 **Apache License 2.0** 授权。  
*(注：TOON 格式和桥接实现的部分内容归功于 TOON 作者。)*

---
© 2026 Kevin Chege. 保留所有权利。  
*Sovereign Systems Lab, Nairobi, Kenya.*
