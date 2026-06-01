[English](README.md) | [简体中文](README.zh-CN.md) | [廣東話](README.yue.md) | [Français](README.fr.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [Suomi](README.fi.md) | [Nederlands](README.nl.md) | [日本語](README.ja.md) | [Русский](README.ru.md) | [한국어](README.ko.md)

# Nairobi Canvas: 立即模式節點圖視覺化編譯器

Nairobi Canvas 係一個用硬體加速嘅視覺化編譯器，專門用嚟整數據處理管道（data pipelines）。佢提供咗一個基於 `egui`/`egui-snarl` 嘅立即模式（immediate-mode）節點圖 UI，可以將視覺化嘅工作流編譯成 GVariant DAG（有向無環圖）格式，再交畀 Nairobi Hub 去行。

## 特色

- **視覺化管道構建器**：用拖放式節點圖界面嚟整數據工作流，簡單直接
- **原生檔案選取器**：喺 Ingest 節點撳個 📂 掣就可以搵 CSV 檔
- **SQL 查詢預設**：內置咗好多查詢模板（執晒所有 Column、執單一 Column、Where Clause、多 Column 查詢等等）
- **GVariant 序列化**：將啲圖編譯做 GVariant 格式，做到零拷貝（zero-copy）IPC
- **拓撲排序**：自動檢查有冇迴圈（cycle detection）同埋排好執行次序

## 節點類型

| 節點 | 輸入 | 輸出 | 描述 |
|------|--------|---------|-------------|
| **Ingest** | 0 | 1 | 經原生檔案選取器嚟讀取 CSV 數據集 |
| **SqlQuery** | 1 | 1 | 對輸入嘅數據行 Polars SQL 查詢 |
| **AxiomCrunch** | 1 | 1 | 計啲統計數據（平均值 mean、標準差 std_dev、峰度 kurtosis） |
| **LagosPlot** | 1 | 0 | 渲染視覺化圖表（折線圖 sparkline、散點圖 scatter、PNG、JPG） |

## 安裝

```bash
pip install nairobi-os
```

或者由源碼自己 Build：
```bash
cargo build --release
# 個 Canvas demo 係一個 Rust binary - 詳情睇返 examples/canvas_compile_demo.rs
```

## 用法

### Rust (Native)

行個 Demo 程式：
```bash
cargo run --example canvas_compile_demo
```

### Python

用裝好咗嘅 Package：
```python
import nairobi_os as nb

# 開個視覺化畫布嚟編譯 DAG
dag_bytes = nb.canvas.open()

# 執行編譯咗嘅管道
if dag_bytes:
    nb.canvas.execute(dag_bytes)
```

或者行個完整嘅測試 Script：
```bash
python test_canvas.py
```

呢個 Script 會做：
1. `nairobi_os.ignite()` - 啟動 Axum Refinery 同埋 Nairobi Hub 嘅守護進程（daemons）
2. `nb.canvas.open()` - 開啟視覺化節點圖編輯器
3. `nb.canvas.execute(dag_bytes)` - 行個編譯咗嘅管道，仲會計埋時

呢個畫布會 Export 一個用 GVariant 編碼嘅 DAG，你可以：
- 經 `nb.canvas.execute()` 嚟行
- 儲存落 Hard disk 第時再用
- 經 D-Bus 或者 Shared memory 嚟傳送

## 點樣整圖

1. 喺畫布個網格度**撳右掣**開個節點選單
2. 揀一個節點類型（Ingest, SQL Query, Axiom Crunch, 或者 Lagos Plot）
3. 由輸出粒點（藍色）拉條線去輸入粒點（綠色）嚟**連接**啲節點
4. 撳 **Compile Graph** 將個工作流序列化

## 執行流程

```
畫布圖表 → GVariant DAG → Nairobi Hub → Axum Refinery / Lagos Vision
```

編譯好嘅 DAG 會經 IPC 傳去 Hub，Hub 會將啲節點分派去：
- **Axum Refinery**：做數據攝取同埋統計處理
- **Lagos Vision**：做硬體加速嘅視覺化渲染

想知更多架構細節同埋系統全貌，請睇返[主存儲庫嘅 README](../README.md)。

## 支持
如果你覺得 Nairobi OS 幫到手，可以考慮支持下我哋：

[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-PayPal-blue.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=chege.finance@gmail.com&item_name=Support+Nairobi+OS+Development)

## 許可證
呢個項目係用 **Apache License 2.0** 授權嘅。

© 2026 Kevin Chege. 版權所有。
