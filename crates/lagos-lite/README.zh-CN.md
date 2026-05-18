[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md)

# Lagos Vision (lagos-lite)

## 概览
Lagos Vision 是 Nairobi OS 的高性能渲染引擎。它旨在通过将分析数据直接内存映射到 GPU 流水线中，以亚毫秒级的延迟可视化数百万个数据点。Lagos 作为一个无头守护进程运行，通过 WebSockets 将 JPEG 编码的帧流传输到 Jupyter 笔记本组件。

## 核心特性
- **零拷贝渲染 (Zero-Copy Rendering)**：数据从 `memfd` 句柄直接内存映射到 `wgpu` 缓冲区。
- **硬件加速**：使用 `egui` 和 `wgpu`（Vulkan, Metal, DX12 或 OpenGL）进行高性能绘图。
- **LTTB 降采样**：在 GPU 上实现最大三角形三桶 (Largest-Triangle-Three-Buckets) 算法，在渲染大规模数据集时保持视觉准确性。
- **事件驱动架构**：空闲时不消耗 CPU；仅在数据更新或用户交互时进行渲染。

## 架构
Lagos Vision 由以下部分组成：
- **Lagos Lite**：提供渲染流水线的核心库。
- **Lagos Vision Daemon**：管理 `wgpu` 表面和 WebSocket 服务器的二进制进程。
- **Lagos Widget**：显示数据流的 `anywidget` Python 组件。

## 安装

### 前置条件
- **GPU**：兼容 Vulkan 的 GPU（或用于软件回退的 OSMesa）。
- **系统库**：`libosmesa6-dev`, `mesa-utils`, `xvfb`。

### 构建
```bash
cargo build --release -p lagos-lite --bin lagos-vision-daemon
```

## 使用

### 在 Nairobi OS 中
Lagos 通常通过 Python 中的 `SovereignFrame.plot()` 方法使用。

### 手动调试
你可以手动启动守护进程来测试渲染流水线：
```bash
./target/release/lagos-vision-daemon --fd <FD_INT> --width 1000 --height 400
```

## 开发

### 实现自定义可视化层
1.  **修改流水线**：在 `src/pipeline.rs` 中定义你的顶点和片段着色器 (WGSL)。
2.  **更新缓冲区布局**：将传入的 `memfd` 数据映射到新着色器的绑定组 (bind groups)。
3.  **UI 集成**：在 `src/device.rs` 中向 `egui` 界面添加控制元素（滑块、按钮）。

### 无头环境
在 Google Colab 等环境中，Lagos 使用 `xvfb-run` 或 OSMesa 来处理缺少物理显示器的情况：
```bash
xvfb-run -s "-screen 0 1024x768x24" ./target/release/lagos-vision-daemon ...
```

## 测试
Lagos 包含视觉集成测试，可以捕获帧并将其与基准图像进行比较。
```bash
cargo test -p lagos-lite
```

## 常见问题排查
- **WebSocket 连接失败**：在云环境（Colab/SageMaker）中，确保代理端口已正确映射。如果检测到 `google.colab`，Nairobi Python 会自动处理此问题。
- **未找到 WGPU 适配器**：确保已安装 GPU 驱动程序。如果使用仅限 CPU 的环境，Lagos 将尝试回退到软件适配器。

## 许可证
该项目根据 **Apache License 2.0** 获得许可。

---
© 2026 Kevin Chege. 保留所有权利。
