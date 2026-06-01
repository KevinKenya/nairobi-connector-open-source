[English](README.md) | [简体中文](README.zh-CN.md) | [廣東話](README.yue.md) | [Français](README.fr.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [Suomi](README.fi.md) | [Nederlands](README.nl.md) | [日本語](README.ja.md) | [Русский](README.ru.md) | [한국어](README.ko.md)

# Nairobi Canvas: イミディエイト・モード・ノードグラフ・ビジュアル・コンパイラ

Nairobi Canvasは、データ処理パイプラインを構築するためのハードウェアアクセラレーション対応ビジュアルコンパイラです。`egui`/`egui-snarl`で構築されたイミディエイト・モードのノードグラフUIを提供し、ビジュアルワークフローをNairobi Hubで実行可能なGVariant DAG（有向非巡回グラフ）形式にコンパイルします。

## 特徴

- **ビジュアル・パイプライン・ビルダー**: データワークフローのためのドラッグ＆ドロップ式ノードグラフ・インターフェース
- **ネイティブ・ファイル・ピッカー**: Ingestノードの📂ボタンをクリックしてCSVファイルをブラウズ
- **SQLクエリ・プリセット**: 事前設定済みのクエリテンプレート（全カラム、単一カラム、Where句、マルチカラム）
- **GVariantシリアライゼーション**: ゼロコピーIPCのためにグラフをGVariant形式にコンパイル
- **トポロジカルソート**: 自動サイクル検出と実行順序の最適化

## ノードタイプ

| ノード | 入力 | 出力 | 説明 |
|------|--------|---------|-------------|
| **Ingest** | 0 | 1 | ネイティブ・ファイル・ピッカー経由でCSVデータセットをロード |
| **SqlQuery** | 1 | 1 | 入力データに対してPolars SQLクエリを実行 |
| **AxiomCrunch** | 1 | 1 | 統計（平均、標準偏差、尖度）を計算 |
| **LagosPlot** | 1 | 0 | ビジュアライゼーション（スパークライン、散布図、PNG、JPG）をレンダリング |

## インストール

```bash
pip install nairobi-os
```

またはソースからビルド：
```bash
cargo build --release
# canvasのデモはRustバイナリです。examples/canvas_compile_demo.rsを参照してください。
```

## 使い方

### Rust (ネイティブ)

デモアプリケーションを実行：
```bash
cargo run --example canvas_compile_demo
```

### Python

インストールされたパッケージを使用：
```python
import nairobi_os as nb

# DAGコンパイルのためにビジュアルキャンバスを開く
dag_bytes = nb.canvas.open()

# コンパイルされたパイプラインを実行
if dag_bytes:
    nb.canvas.execute(dag_bytes)
```

または完全なテストスクリプトを実行：
```bash
python test_canvas.py
```

このスクリプトは以下を実行します：
1. `nairobi_os.ignite()` - Axum RefineryとNairobi Hubデーモンを起動
2. `nb.canvas.open()` - ビジュアルノードグラフエディタを起動
3. `nb.canvas.execute(dag_bytes)` - タイミングメトリクスと共にコンパイルされたパイプラインを実行

キャンバスはGVariantエンコードされたDAGをエクスポートし、以下のことが可能です：
- `nb.canvas.execute()`経由で実行
- 後で使用するためにディスクに保存
- D-Bus/共有メモリ経由で送信

## グラフの構築

1. キャンバスのグリッド上で**右クリック**してノードメニューを開く
2. ノードタイプ（Ingest、SQL Query、Axiom Crunch、またはLagos Plot）を選択
3. 出力ピン（青）から入力ピン（緑）へドラッグしてノードを**接続**
4. **Compile Graph**をクリックしてワークフローをシリアライズ

## 実行フロー

```
Canvas Graph → GVariant DAG → Nairobi Hub → Axum Refinery / Lagos Vision
```

コンパイルされたDAGはIPC経由でHubに送信され、Hubはノードを以下にルーティングします：
- **Axum Refinery**: データの取り込みと統計処理
- **Lagos Vision**: ハードウェアアクセラレーションによるビジュアライゼーションのレンダリング

アーキテクチャの詳細とシステム全体の概要については、[メインリポジトリのREADME](../README.md)を参照してください。

## サポート
Nairobi OSが役立つと思われた場合は、プロジェクトへの支援をご検討ください：

[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-PayPal-blue.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=chege.finance@gmail.com&item_name=Support+Nairobi+OS+Development)

## ライセンス
このプロジェクトは **Apache License 2.0** の下でライセンスされています。

© 2026 Kevin Chege. All Rights Reserved.
