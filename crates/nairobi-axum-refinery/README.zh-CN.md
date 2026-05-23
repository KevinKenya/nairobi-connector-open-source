[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [Русский](README.ru.md) | [한국어](README.ko.md)

# Nairobi Axum Refinery

## 概览
Axum Refinery 是 Nairobi OS 的高性能核心。它使用 Rust 编写，旨在通过绕过内核的 I/O 和向量化并行分析来充分发挥现代硬件的性能。它作为一个 D-Bus 服务运行，管理摄取到匿名内存文件描述符 (`memfd`) 中的数据生命周期。

## 核心特性
- **Dirac 摄取引擎**：一个 3 级摄取策略，使用 `io_uring` (1 级)、`copy_file_range` (2 级) 和 `mmap` (3 级)。
- **Axiom Crunch**：由 Polars 和 Rayon 驱动的向量化统计矩计算（均值、方差、偏度、峰度）。
- **Relational Strike**：优化的 Pearson 和 Spearman 相关系数计算。
- **SQL 分析**：使用 `polars-sql` 在内存驻留数据上直接执行 SQL 查询。
- **零拷贝数据平面 (Zero-Copy Data Plane)**：通过 `iceoryx2` 共享内存和 D-Bus 公开分析结果。

## 架构
Refinery 被划分为多个专用引擎：
- `DiracEngine`：处理硬件加速的 I/O。
- `AnalyzeEngine`：执行统计计算和 SQL 执行。
- `DbusService`：实现 `org.nairobi.NairobiAxumRefinery1` 接口。

## 安装

### 前置条件
- **内核**：Linux 5.10+ (支持 WSL2)。
- **依赖项**：`libdbus-1-dev`, `pkg-config`。
- **Huge Pages**：启用 1GB Huge Pages 时引擎表现最佳。
    ```bash
    echo 1 | sudo tee /sys/kernel/mm/hugepages/hugepages-1048576kB/nr_hugepages
    ```

### 构建
```bash
cargo build --release -p nairobi-axum-refinery
```

## 开发

### 内核级配置
贡献者应注意，`DiracEngine` 尝试使用 `IORING_SETUP_SQPOLL`。为了在没有 root 权限的情况下运行，你可能需要调整 `/proc/sys/kernel/unprivileged_userns_clone` 或以 `CAP_SYS_ADMIN` 权限运行。

### 教程：添加新的统计指标
1.  **定义指标**：在 `src/analyze.rs` 中，更新 `StatisticalProfile` 结构体及其 `compute` 方法。
2.  **更新协议**：在 `crates/nairobi-protocol/src/types.rs` 中的 `DistilledAnalytics` 结构体中添加新字段。
3.  **通过 D-Bus 导出**：确保 `src/dbus_service.rs` 中的 D-Bus 接口能正确序列化更新后的配置文件。

## 测试
Refinery 使用 `tokio::test` 进行异步集成测试。
```bash
cargo test -p nairobi-axum-refinery
```

#### 用于隔离测试的模拟 (Mocking)
你可以通过手动创建 `memfd` 并将其传递给引擎来隔离测试 `AnalyzeEngine`，从而绕过 D-Bus 层：
```rust
let opts = memfd::MemfdOptions::default();
let mfd = opts.create("test.csv")?;
// 写入测试数据...
let engine = AnalyzeEngine::new()?;
let results = engine.analyze(mfd.into_fd(), "target_column")?;
```

## 常见问题排查
- **`io_uring` 初始化失败**：检查你的内核是否支持 `io_uring` (`zgrep CONFIG_IO_URING /proc/config.gz`)。
- **Huge Page 分配失败**：确保主机有足够的连续内存可用。检查 `grep Huge /proc/meminfo`。

## 支持
如果您觉得 Nairobi OS 对您有所帮助，请考虑支持该项目：

[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-PayPal-blue.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=chege.finance@gmail.com&item_name=Support+Nairobi+OS+Development)

## 许可证
该项目根据 **Apache License 2.0** 获得许可。

---
© 2026 Kevin Chege. 保留所有权利。
