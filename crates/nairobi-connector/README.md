[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [Русский](README.ru.md) | [한국어](README.ko.md)

# Nairobi Connector

## Overview
**Nairobi Connector** is the AT-SPI2 semantic bridge and Model Context Protocol (MCP) server for Nairobi OS. It enables **"Computer Use without pixels"** by exposing the Linux desktop's accessibility tree to LLMs and AI agents in a hyper-dense, token-optimized format known as TOON (Token-Oriented Object Notation). By operating purely on semantic UI nodes rather than pixels, it achieves near-instant action dispatch and drastically reduces token overhead.

## Key Features
- **Computer Use Without Pixels**: Interacts directly with the Linux desktop through AT-SPI2, bypassing the need for screenshots, OCR, or visual processing.
- **TOON Compression Algorithm**: Translates raw D-Bus accessibility trees into a highly compressed Markdown representation. It filters out non-interactive "noise" nodes and assigns sequential IDs to actionable elements, yielding < 500 tokens of output in < 50ms.
- **MCP Server Integration**: Implements a robust `rmcp`-based server exposing semantic tools natively to compatible LLM agents.
- **Safe Session Lifecycle**: Features a heartbeat watcher to prevent OS paralysis by auto-releasing `RegistryLock` if the `stdio` pipe hangs.

## Architecture
The connector acts as a bidirectional bridge between LLMs (via MCP) and the Linux desktop (via AT-SPI2/D-Bus). It wraps the `NeuralSession` layer, managing window discovery, UI tree traversal, and localized action injection.

### MCP Tools Provided
- `nairobi_find_window`: Finds and targets a window by title substring (case-insensitive).
- `nairobi_get_ui_map`: Returns the current UI accessibility tree as a TOON-compressed map. Generates a dense listing of interactive elements (buttons, entries, checkboxes) with sequential `[ID: N]` tags for targeting.
- `nairobi_interact`: Executes semantic actions (`click`, `activate`, `focus`) on a UI element using its TOON node ID.
- `nairobi_type_text`: Atomically injects text into an editable field (Entry, TextArea) identified by its TOON node ID.

## Usage

Agents using the Nairobi Connector should follow this fundamental loop:
1. Target a window using `nairobi_find_window`.
2. Observe the current state via `nairobi_get_ui_map`.
3. Read the TOON `[ID: N]` of the desired interactive element.
4. Execute an action on that element via `nairobi_interact` or `nairobi_type_text`.
5. Repeat from step 2 to get fresh IDs before interacting again.

## Support
If you find Nairobi OS useful, consider supporting the project:

[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-PayPal-blue.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=chege.finance@gmail.com&item_name=Support+Nairobi+OS+Development)

## License
This project is licensed under the **Apache License 2.0**.
*(Note: Portions of the TOON format and bridge implementation are credited to The TOON Authors.)*
