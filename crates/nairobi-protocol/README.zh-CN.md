[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md)

# Nairobi 协议 (Nairobi Protocol)

## 概览
Nairobi Protocol crate 定义了在整个 Nairobi OS 生态系统中使用的共享 D-Bus 接口、GVariant 签名和数据结构。它是 Rust 核心、Hub 协调器和 Python 绑定之间类型安全的“单一事实来源”。

## 核心组件
- **接口定义**：服务名称、对象路径和方法签名的常量。
- **共享类型**：与 GVariant 兼容的结构体，如 `DistilledAnalytics` 和 `CorrelationResult`。
- **内存管理**：用于 `memfd` 操作的 `MemoryPipe` 包装器和 `iceoryx2` arena 定义。

## D-Bus 接口
- **服务名称**：`org.nairobi.NairobiAxumRefinery1`
- **对象路径**：`/org/nairobi/NairobiAxumRefinery1`
- **接口**：`org.nairobi.NairobiAxumRefinery1`

## 使用
将此 crate 添加为任何需要在 Nairobi OS 生态系统中进行通信的组件的依赖项。

## 开发
对该 crate 的更改应极其谨慎，因为它们需要重新编译所有依赖的 crate，并且可能会破坏 refinery 与 Python 绑定之间的二进制兼容性。

## 测试
集成测试确保 GVariant 签名与预期的 D-Bus 协议匹配：
```bash
cargo test -p nairobi-protocol
```

## 支持
如果您觉得 Nairobi OS 对您有所帮助，请考虑支持该项目：

[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-PayPal-blue.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=chege.finance@gmail.com&item_name=Support+Nairobi+OS+Development)

## 许可证
该项目根据 **Apache License 2.0** 获得许可。

---
© 2026 Kevin Chege. 保留所有权利。
