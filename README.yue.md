[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [Русский](README.ru.md) | [한국어](README.ko.md) | [Finnish](README.fi.md) | [Cantonese](README.yue.md) | [Français](README.fr.md) | [Nederlands](README.nl.md)

# Nairobi OS：高性能、零拷貝 AI 與數據科學基礎設施

[![PyPI Version](https://img.shields.io/pypi/v/nairobi-os.svg)](https://pypi.org/project/nairobi-os/)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)
![System](https://img.shields.io/badge/Kernel-Linux_6.17_Native-orange.svg)
![Arch](https://img.shields.io/badge/Architecture-x86__64_/_ARM64-red.svg)

---

## 起源：從熔爐到金屬

Nairobi OS 唔係誕生喺舒適嘅企業孵化器或者風投支持嘅研究實驗室。佢係絕對必然嘅產物，源於一系列深刻嘅個人危機，以及喺標準行業工具失效時堅持執行嘅無情驅動力。

我係 Kevin Chege，Sovereign Systems Lab（肯尼亞奈洛比）嘅創始人。從 2009 年到 2022 年，我嘅生活被嚴重嘅酗酒吞噬。佢令我付出咗職業聲譽、機遇，甚至幾乎係生命嘅代價。喺我酒癮最嚴重嘅時期，我喺英國米爾頓凱恩斯開放大學（The Open University）策略辦公室擔任分析師，之前曾擔任 AIESEC 盧旺達創始人兼總裁（2006-2010年）。今日，係我持續清醒嘅第四年。

```
                     LEGIO XIII GEMINA
              "第十三軍團 — 六月十三日"
     失去咗十三年。要用十三年攞返晒返嚟。
```

我嘅編程之路植根於低級系統架構同極端優化。2015 年，我喺[呢篇關於肯尼亞矽谷嘅論文](https://www.linkedin.com/pulse/building-kenyas-silicon-valley-making-work-kevin-chege/)入面闡述咗喺非洲大陸建立去中心化、高度技術性能力嘅願景。當 2023 年 LLM 淘金熱開始嗰陣，我好早就參與其中。我構建並部署咗 LLM 封裝器（wrappers），但好快就意識到佢哋嘅局限性，正如呢份早期嘅 [2023 年 LLM 封裝器演示](https://www.linkedin.com/feed/update/urn:li:activity:7102930955807449088/)所記錄。

我意識到，喺唔穩定嘅 API 之上構建高級封裝器係架構上嘅死胡同。真正嘅戰爭係喺本地硬件限制同資源分配嘅交匯處進行嘅。

喺整個 2025 年，我一直生活喺一部硬件配置高度受限嘅 Lenovo X13 ThinkPad 上：

```
處理器: AMD Ryzen 5 PRO 4650U (6 核, 12 線程)
顯卡: AMD Radeon RX Vega 6 集顯 (iGPU)
內存: 32 GB RAM (系統佔用率極高)
存儲: 256 GB NVMe (99% 已滿)
```

就喺呢部機上，我在 2025 年開發咗 **Tumz** ([Sarafakai](http://www.sarafakai.com))，呢係一款氣隙隔離（air-gapped）、零延遲嘅臨床決策支持 AI。佢喺集顯（iGPU）上同時執行實時語音轉錄同臨床推理，將整個統一醫學語言系統（UMLS）長駐喺內存入面。我哋目前正與肯尼亞嘅一間醫院合作，試用 Tumz 進行為期一年嘅臨床試驗——因為人類健康需要嚴格嘅實證檢驗，而唔係開發者嘅臆測。

喺開發 Tumz 嘅過程中，我遇到咗現代數據科學技術棧巨大且系統性嘅低效問題：
1. **Python 稅**：端到端內存複製、GIL 瓶頸以及巨大嘅運行時開銷。
2. **瀏覽器稅**：Manifest V3 嘅複雜性、渲染延遲以及長時間運行嘅智能體（agent）對話中嘅高頻通信故障。
3. **OS 內核瓶頸**：低效嘅進程調度、CPU 線程飢餓以及顯示服務器開銷（Wayland 與 X11 上下文切換）。

所以，喺 2025 年底，我著手構建一個完全繞過呢啲限制嘅底層基礎架構棧——一個專為零拷貝數據管道同硬件原生 AI 執行而設計嘅智能體操作系統（Agentic Operating System）。呢個倉庫正係該引擎嘅開源核心。

---

## 戰績：過去一年嘅 9,180 次貢獻

現代開發社區嘅某啲批評家望向啲新、高度先進嘅項目，就會將佢哋 dismiss 做 "AI 生成嘅模板代碼"。俾這啲 skeptics 我展示 commit 日誌嘅 raw、物理證據。

我 my other github profile (https://github.com/ChegeKenya) 做咗一個 intense、daily 系統工程嘅 empirical 記錄。喺 2025 年 alone 就有 7,888 次貢獻。喺 2026 年頭五個月又 addition 咗 1,420 次。總共就有 9,180 次貢獻喺過去 365 天——嗰個 near-continuous 嘅 green commits 序列，橫跨 low-latency Rust runtimes、臨床 AI 管道同 zero-copy shared memory 系統。呢段代碼喺戰地寫㗎、喺 bare metal 上 compile，在 byte 字节層面睇過。

```
2025: [██████████████████████████████████████████████████] 7,888 Commits
  2026: [██████████] 1,420 Commits
  Total (Last Year): 9,180 Commits of Pure Systems Code
```

### 硬件原生開發約束

---

## 全球採用與遙測

Nairobi OS 喺 2026 年 5 月 6 日發佈，迅速獲得咗全球系統程序員、量化研究員同邊緣計算架構師嘅青睞。呢啲下載統計數據源自實時 [ClickPy Nairobi OS 儀表板](https://clickpy.clickhouse.com/dashboard/nairobi-os)，你可以親自搜索同探索呢啲指標。

### 全球累計分佈 (2026年5月6日 – 2026年5月23日)

| 指標 | 測量值 | 背景信息 |
| :--- | :--- | :--- |
| **全球排名** | **#75,293** | 在 PyPI 上 797,894 個活躍包中 |
| **百分位** | **9.43%** | 系統級 Python 擴展嘅頂級排名 |
| **總下載量** | **1,525** | 乾淨、有機、高意向嘅開發者下載 |

### 各版本下載量

```
  0.2.0 [████████████████████████████████████████] 342
  0.2.1 [██████████████████████████] 224
  0.3.0 [████████████████████████] 212
  0.3.1 [████████████████████] 176
  0.1.0 [███████████████████] 169
  0.4.1 [██████████████] 120
```

### 排名前 10 嘅採用主權地區

| 排名 | 地區 | 國家代碼 | 下載量 |
| :--- | :--- | :--- | :--- |
| 1 | 美國 | US | 661 |
| 2 | 中國香港 | HK | 103 |
| 3 | 中國大陸 | CN | 84 |
| 4 | 德國 | DE | 74 |
| 5 | 日本 | JP | 65 |
| 6 | 新加坡 | SG | 56 |
| 7 | 英國 | GB | 51 |
| 8 | 法國 | FR | 51 |
| 9 | 俄羅斯 | RU | 42 |
| 10 | 韓國 | KR | 30 |

---

## 支持與主權

如果 Nairobi OS 優化咗你嘅數據管道、降低咗你嘅雲服務賬單，或驅動咗你嘅本地智能體架構，請考慮支持我哋嘅獨立系統研究。每一筆貢獻都將直接用於奈洛比嘅硬件級編譯器優化同邊緣計算測試。

[![支持 Nairobi OS 開發](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-PayPal-blue.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=chege.finance@gmail.com&item_name=Support+Nairobi+OS+Development)

---

## 核心特性

* **無像素計算機使用**：繞過緩慢、昂貴嘅基於視覺嘅智能體管道。通過 AT-SPI2 同 TOON（面向 Token 嘅對象表示法）壓縮算法與 Linux 桌面原生交互，直接將原始層次樹饋送畀 LLM。
* **零拷貝數據接入**：利用 `io_uring` 同 1GB 大頁（Huge Pages）實現硬件加速、內核旁路嘅快速數據加載。
* **硬件加速視覺化**：基於 `wgpu` 同 `egui` 構建嘅 `lagos-lite` 渲染守護進程，提供低延遲、交互式嘅 Jupyter 繪圖。
* **向量化分析執行**：利用 Polars 查詢執行同 Rayon 多線程數據管道實現極端嘅 CPU 飽和度。
* **主權接口**：一個流暢嘅 Python API (`SovereignFrame`)，封裝咗內存映射文件描述符同進程間通信（IPC）。

---

## 開源與企業級架構

Nairobi OS 喺結構上係分叉嘅。開源倉庫提供基礎嘅高性能數據處理同單節點視覺化基元。閉源嘅企業級生態系統則包含先進嘅多智能體、高可用性以及特定行業嘅實現。

```
                                  +---------------------------------------+
                                  |         Nairobi Python API            |
                                  +---------------------------------------+
                                                      |
                                     [ 通過 D-Bus / 共享內存傳輸 GVariant ]
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
     |     Axum Refinery (數據)     | <===[ 零拷貝 IPC / iceoryx2 ]===> |     Lagos Vision (視覺)      |
     +------------------------------+                                    +------------------------------+
```

### 開源 Crate 工作區 (`crates/`)

1. **`nairobi-axum-refinery`**：高性能 Rust 守護進程，管理原始數據接入、Rayon 並行化統計計算以及 Polars 向量化查詢執行。
2. **`nairobi-hub`**：中央 IPC 協調器。管理並路由客戶端與 refinery 守護進程之間嘅文件描述符同信號。
3. **`lagos-lite`**：視覺皮層。一個無頭、事件驅動嘅渲染引擎，將內存映射文件直接映射到 GPU 管道中。
4. **`nairobi-protocol`**：共享協議層。定義標準嘅 GVariant 序列化方案、錯誤類型同共享內存佈局。
5. **`nairobi-python`**：通過 `PyO3` 編譯並使用 `Maturin` 打包嘅 Python 擴展模塊。

### 私有企業生態系統 (`modules/`)

我哋嘅企業級組件保存在私有倉庫（`Sovereign-Systems-Lab`）入面，面向工業、金融同國家級基礎設施進行授權。

1. **`sovereign-ui`**：企業級 AT-SPI2 引擎。實現 Aegis 協議安全、硬件綁定以及生產級桌面操作。
2. **`nairobi-connector`**：高級模型上下文協議（MCP）服務器，管理企業 LLM 嘅原始、低延遲 D-Bus 信號。
3. **`tactical-rtos-node`**：用於安全關鍵型邊緣工業自動化嘅超低延遲、實時操作系統調度器。
4. **`industrial-guardian-rust` / `industrial-guardian-python`**：自主站點可靠性工程（SRE）層，具有預測性 OOM、內存洩漏同系統崩潰規避功能。
5. **`fintech-bridge-rust`**：實時高頻交易解析器同傳統主機橋接器（EBCDIC/SBA 終端解析）。
6. **`aviation-audio-rust`**：亞毫秒級、無鎖音頻流處理、聲學遙測分析同原始波形 DSP。
7. **`drawbridge_api`**：安全、經過身份驗證嘅多租戶 gRPC 吊橋，將本地內核與唔受信任嘅雲智能體調用隔離。

### 能力對比矩陣

| 能力 / 特性 | 開源核心 (`crates/`) | 企業套件 (`modules/`) |
| :--- | :---: | :---: |
| **接入引擎** | `mmap` / `copy_file_range` | `io_uring` + `SQPOLL` + 1GB 大頁 |
| **統計分析** | 基礎描述性統計 | 向量化、多通道偏度/峰度、相關性分析 |
| **查詢引擎** | 進程內 Polars SQL | 分布式 Apache Arrow / DataFusion 集群 |
| **IPC 機制** | POSIX 共享內存 / D-Bus | 零拷貝 `iceoryx2` 共享內存域 |
| **視覺化** | 本地 Jupyter `anywidget` | WebRTC GStreamer / 透明 Wayland Layer-Shell 覆蓋 |
| **安全與合規** | 標準 POSIX 邊界 | Aegis 協議，SHA-256 鏈式取證賬本 |
| **身份驗證** | 無（本地受信任用戶） | 硬件綁定（TPM 2.0 / CPU ID），私有 PKI |
| **平台目標** | 單節點 Linux | 分布式雲 / 邊緣節點 / 高頻交易 |

---

## 安裝與設置

### 系統要求
- **操作系統**：Linux（推薦 Ubuntu 22.04+）或 Windows Subsystem for Linux (WSL2)。
- **顯卡**：兼容 Vulkan、Metal 或 OpenGL 嘅驅動程序。
- **Python**：3.10 或更高版本。
- **Rust**：穩定嘅工具鏈（如果從源碼構建）。

### 快速安裝 (PyPI)
```bash
pip install nairobi-os
```

### 從源碼構建
要編譯整個工作區，包括原生守護進程同 Python 擴展：

1. **克隆倉庫**：
   ```bash
   git clone https://github.com/KevinKenya/nairobi-connector-open-source.git
   cd nairobi-connector-open-source
   ```

2. **配置虛擬環境**：
   ```bash
   python3 -m venv .venv
   source .venv/bin/activate
   pip install maturin pyo3-build-config zbus anywidget traitlets pandas
   ```

3. **執行工作區構建**：
   ```bash
   chmod +x build_wheel.sh
   ./build_wheel.sh --release
   ```
   呢將編譯原生守護進程，將其複製到包目錄，並喺 `crates/nairobi-python/target/wheels/` 下構建 wheel 包。

---

## 使用指南

### 1. 數據分析 (內存管道)

Nairobi OS 提供咗 `SovereignFrame` API。佢喺底層處理原始內存映射，從而實現快速嘅數據操作。

```python
import nairobi_os as nb

# 啟動背景 refinery 守護進程
nb.connect()

# 使用零拷貝內存管道接入數據集
frame = nb.read_csv("simulator/fndds_ingredient_nutrient_value.csv")

# 通過 Rust refinery 執行向量化計算
profile = frame.crunch("value")
print(f"均值: {profile['mean']:.4f}")
print(f"標準差: {profile['std_dev']:.4f}")

# 直接喺內存映射嘅 frame 上執行任意 SQL 查詢
subset = frame.query("SELECT * FROM dataset WHERE value > 50.0")

# 喚起 Lagos 加速嘅交互式繪圖小部件
subset.plot(column="value")
```

### 2. 無像素計算機使用 (MCP)

要使用 AT-SPI2 語義接口，你嘅 AI 智能體應與公開嘅 MCP 服務器工具進行交互，而唔係讀取屏幕截圖：

```
                     計算機使用序列
                     
  [ LLM 智能體 ]                                [ Nairobi OS ]
        |                                             |
        |===> nairobi_find_window("Text Editor") ====>| (定位目標)
        |<=== 返回窗口 ID 和邊界 =====================|
        |                                             |
        |===> nairobi_get_ui_map() ==================>| (生成 TOON)
        |<=== 返回壓縮嘅 Markdown 樹 =================|
        |     "[ID: 12] Button: 'Save'"               |
        |                                             |
        |===> nairobi_interact(12, "click") =========>| (執行操作)
        |<=== 返回成功狀態 ===========================|
```

---

## 系統微調 (貢獻者指南)

為了達到我哋基准測試中所示嘅性能配置文件，你嘅主機內核必須配置為系統級內存映射。

### 1GB 大頁 (Huge Pages)
Nairobi OS 使用 1GB 大頁來繞過 CPU 喺海量數據集上嘅轉換旁路緩衝區（TLB）轉換開銷。

要在 Linux 主機上分配一個大頁：
```bash
echo 1 | sudo tee /sys/kernel/mm/hugepages/hugepages-1048576kB/nr_hugepages
```
*注意：如果系統由於碎片化而無法分配 1GB 頁面，引擎會自動回退到透明大頁 (THP)。*

### D-Bus Broker 配置
喺高頻環境入面，請確保安裝咗 `dbus-broker` 而唔係傳統嘅 `dbus-daemon`，以便喺控制平面上實現快速嘅信號傳播。

---

## 許可證

本項目採用 **Apache License 2.0** 授權。  
*(注：TOON 格式和橋接實現嘅部分內容歸功於 TOON 作者。)*

---
© 2026 Kevin Chege. 保留所有權利。  
*Sovereign Systems Lab, Nairobi, Kenya.*
