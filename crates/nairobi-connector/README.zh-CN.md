[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md)

# Nairobi Connector

## 概述
**Nairobi Connector** 是 Nairobi OS 的 AT-SPI2 语义桥接器和模型上下文协议 (MCP) 服务器。它通过将 Linux 桌面的无障碍树 (Accessibility Tree) 以超高密度、优化 token 的 TOON (Token-Oriented Object Notation) 格式暴露给 LLM 和 AI 代理，实现了**“无像素的计算机使用”** (Computer Use without pixels)。通过纯粹操作语义 UI 节点而不是像素，它实现了近乎即时的操作分发，并大幅降低了 token 消耗。

## 主要特性
- **无像素的计算机使用**：直接通过 AT-SPI2 与 Linux 桌面交互，完全无需截图、OCR 或任何视觉处理过程。
- **TOON 压缩算法**：将原始的 D-Bus 无障碍树转换为高度压缩的 Markdown 表示形式，过滤非交互式的“噪音”节点，并为可操作元素分配连续 ID。
- **MCP 服务器集成**：实现基于 `rmcp` 的强大服务器，向兼容的 LLM 代理原生暴露语义工具。
- **安全的会话生命周期**：具有心跳监视功能，以防 `stdio` 管道挂起导致操作系统瘫痪。

## 架构
该连接器充当 LLM（通过 MCP）和 Linux 桌面（通过 AT-SPI2/D-Bus）之间的双向桥梁。它封装了 `NeuralSession` 层，管理窗口发现、UI 树遍历和局部操作注入。

### 提供的 MCP 工具
- `nairobi_find_window`：通过标题子字符串查找并定位窗口（不区分大小写）。
- `nairobi_get_ui_map`：返回当前 UI 无障碍树作为 TOON 压缩映射。生成具有顺序 `[ID: N]` 标记的交互式元素的密集列表以供定位。
- `nairobi_interact`：使用其 TOON 节点 ID 在 UI 元素上执行语义操作（`click`、`activate`、`focus`）。
- `nairobi_type_text`：自动将文本注入到由其 TOON 节点 ID 标识的可编辑字段（Entry、TextArea）中。

## 使用方法
使用 Nairobi Connector 的代理应遵循以下基本循环：
1. 使用 `nairobi_find_window` 锁定目标窗口。
2. 通过 `nairobi_get_ui_map` 观察当前状态。
3. 读取所需交互元素的 TOON `[ID: N]`。
4. 通过 `nairobi_interact` 或 `nairobi_type_text` 对该元素执行操作。
5. 从步骤 2 重复，以在下次交互前获取最新的 ID。

## 支持
如果您觉得 Nairobi OS 对您有所帮助，请考虑支持该项目：

[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-PayPal-blue.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=chege.finance@gmail.com&item_name=Support+Nairobi+OS+Development)

## 许可证
本项目采用 **Apache License 2.0** 许可证。
*(注：TOON 格式和桥接实现的部分内容归 TOON 作者所有。)*
