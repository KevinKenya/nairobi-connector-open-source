[English](README.md) | [简体中文](README.zh-CN.md)

# Nairobi Hub

## 概览
Nairobi Hub 是 Nairobi OS 的核心 IPC（进程间通信）协调器。它负责在高性能 Rust refinery 及其客户端之间协调文件描述符、D-Bus 信号和共享内存段。

## 核心特性
- **文件描述符代理 (FD Proxying)**：使用 GVariant 签名通过 D-Bus 安全地传递 `memfd` 文件描述符。
- **服务管理**：监控和管理 `org.nairobi.NairobiAxumRefinery1` 的生命周期。
- **混合数据平面**：通过 `iceoryx2` 共享内存（为了性能）或 D-Bus（为了兼容性）动态路由数据。
- **语义解码**：将原始二进制分析结果解码为人类可读的报告和原生 Python 结构。

## 架构
Hub 被划分为多个内部模块：
- `client.rs`：D-Bus 代理客户端。
- `shm_subscriber.rs`：处理 `iceoryx2` 共享内存订阅。
- `decoder.rs`：将 GVariant 结果转换为 Markdown 和 JSON。

## 使用
Hub 主要作为 `nairobi-python` 使用的一个库，用于与 refinery 通信。

## 开发
修改 Hub 时，请确保 D-Bus 接口的任何更改也反映在 `nairobi-protocol` 中。

## 测试
Hub 的集成测试验证了完整的 IPC 往返：
```bash
cargo test -p nairobi-hub
```

## 许可证
该项目根据 **PolyForm Noncommercial License 1.0.0** 获得许可。

---
© 2026 Kevin Chege. 保留所有权利。
