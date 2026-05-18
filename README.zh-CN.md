[English](README.md) | [简体中文](README.zh-CN.md)

# Nairobi OS

## 概览
Nairobi OS 是一个高性能、分布式的数据科学基础设施，专为极高的资源效率而设计。它利用专门的基于 Rust 的 refinery 守护进程，能够在受限环境（边缘计算、物联网、无服务器架构）中处理大规模数据集。通过利用 `io_uring`、`memfd` 和 Huge Pages 等内核级特性，Nairobi OS 实现了亚毫秒级的 IPC 开销和零拷贝数据流水线。

## 核心特性
- **零拷贝摄取 (Zero-Copy Ingestion)**：使用 `io_uring` 和 1GB Huge Pages 进行硬件加速的数据加载。
- **硬件加速可视化**：通过 Lagos Vision 引擎（`wgpu` 和 `egui`）实现交互式 Jupyter 绘图。
- **融合分析流水线 (Fused Analytics Pipeline)**：在单次 D-Bus 往返中完成数据摄取、处理和关联。
- **绕过内核的性能 (Kernel-Bypass Performance)**：利用 Polars 和 Rayon 进行向量化分析，以实现最大的硬件饱和度。
- **Sovereign 接口**：一个流畅的 Python API，隐藏了底层 IPC 和内存管理的复杂性。

## 架构
Nairobi OS 构建在通过 D-Bus 和共享内存连接的三个专用组件之上：

1.  **Nairobi Axum Refinery**：高性能 Rust 核心。管理原始数据摄取和并行分析。
2.  **Nairobi Hub**：IPC 协调器。协调 refinery 与客户端之间的文件描述符和信号。
3.  **Lagos Vision**：视觉核心。一个无头渲染引擎，将 `memfd` 句柄直接映射到 GPU 流水线中。
4.  **Nairobi Python**：高级桥梁。为 Rust 生态系统提供 Python 化接口。

```text
[ 数据源 ] -> (io_uring/Huge Pages) -> [ Axum Refinery ]
                                                  |
                                          (D-Bus / memfd / iceoryx2)
                                                  |
                                          [ Nairobi Hub ]
                                             /        \
                             [ Nairobi Python ]    [ Lagos Vision ]
                                     |                    |
                            [ Jupyter Notebook ] <-> [ 视觉输出 ]
```

## 安装

### 前置条件
- **操作系统**：Linux 或 WSL2（`io_uring` 和 `memfd` 需要内核 5.10+）。
- **Rust**：1.70+
- **Python**：3.10+
- **系统库**：
    ```bash
    sudo apt-get update && sudo apt-get install -y \
        build-essential \
        pkg-config \
        libdbus-1-dev \
        python3-dev \
        dbus-x11 \
        libosmesa6-dev \
        mesa-utils
    ```

### 从源码构建
1. **克隆仓库**：
    ```bash
    git clone https://github.com/KevinKenya/nairobi-connector-open-source
    cd nairobi-connector-open-source
    ```

2. **设置虚拟环境**：
    ```bash
    python3 -m venv .venv
    source .venv/bin/activate
    pip install maturin pyo3-build-config zbus anywidget traitlets
    ```

3. **构建整个技术栈**：
    ```bash
    ./build_wheel.sh
    ```

4. **安装 Wheel 包**：
    ```bash
    pip install target/wheels/nairobi_os-0.3.1-py3-none-any.whl
    ```

## 系统配置（贡献者指南）

### Huge Pages
Refinery 引擎优先使用 1GB Huge Pages 作为零拷贝缓冲区。要在主机上启用这些页面：
```bash
echo 1 | sudo tee /sys/kernel/mm/hugepages/hugepages-1048576kB/nr_hugepages
```
*注意：如果 1GB 页面不可用，引擎将自动回退到透明大页 (THP)。*

### io_uring 和 SQPOLL
`DiracEngine` 使用带有 `SQPOLL` 的 `io_uring` 以获得最大的 I/O 吞吐量。`SQPOLL` 通常需要提升权限（`CAP_SYS_ADMIN`）或配置了 `IORING_SETUP_SQPOLL` 的内核。如果引擎无法初始化 `SQPOLL`，它将回退到标准的 `io_uring` 模式。

## 使用
```python
import nairobi_os

# 启动 refinery
nairobi_os.connect()

# 将数据摄取到 SovereignFrame
df = nairobi_os.read_csv("dataset.csv")

# 执行向量化分析
print(f"Mean: {df.column_name.mean()}")

# 启动交互式可视化
df.plot()
```

## 测试
Nairobi OS 包含一个全面的测试套件，涵盖了 Rust 单元测试、IPC 集成和 Python 绑定。

### 运行所有测试
```bash
# 运行 Rust 测试
cargo test --workspace

# 运行 Python 集成测试
python3 test_nairobi.py
```

### 基准测试
可以从 `nairobi-benchmarks` 目录运行详细的性能基准测试：
```bash
cd nairobi-benchmarks
pip install -r requirements.txt
python orchestration/benchmark_runner.py --workload workloads/workload_nba_pipeline.yaml
```

## 常见问题排查
- **D-Bus 连接被拒绝**：确保 `dbus-daemon` 正在运行。在无头环境中，使用 `dbus-launch`。
- **Lagos 渲染问题**：Lagos 需要有效的 GPU 驱动程序或 OSMesa 进行软件回退。使用 `glxinfo` 进行验证。
- **Huge Page 分配失败**：检查 `/proc/meminfo` 以确保内核预留了足够的 huge pages。

## 许可证
该项目根据 **PolyForm Noncommercial License 1.0.0** 获得许可。个人、教育和研究用途免费。

---
© 2026 Kevin Chege. 保留所有权利。
