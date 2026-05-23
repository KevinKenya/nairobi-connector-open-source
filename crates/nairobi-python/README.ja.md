[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [Русский](README.ru.md) | [한국어](README.ko.md)

# Nairobi OS (nairobi-python)

## 概要
**Nairobi OS** は、極限の資源効率を目指して設計された、高性能で分散型の AI およびデータサイエンスインフラストラクチャです。特化した Rust ベースの refinery デーモンを活用することで、制約の厳しい環境（エッジ、IoT、サーバーレス）における大規模なデータセットの処理を可能にし、MCP 準拠のアクセシビリティブリッジを介した**「ピクセルなしのコンピュータ使用」**を提供します。

`io_uring`、`memfd`、ヒュージページ (Huge Pages) などのカーネルレベルの機能を利用することで、Nairobi OS はサブミリ秒の IPC オーバーヘッドとゼロコピーのデータパイプラインを実現します。

## 主な機能
- **ピクセルなしのコンピュータ使用**：AT-SPI2 と TOON (Token-Oriented Object Notation) 圧縮アルゴリズムを介して Linux デスクトップと直接相互作用し、AI エージェントに対する OCR や視覚的処理を完全にバイパスします。
- **ゼロコピーデータ接入**：`io_uring` と 1GB ヒュージページを利用した、ハードウェア加速された超高速データロード。
- **ハードウェア加速ビジュアライゼーション**：Lagos Vision レンダリングエンジン（`wgpu` と `egui`）を介した、低遅延でインタラクティブな Jupyter プロッティング。
- **融合された分析パイプライン**：単一の D-Bus 往復で、データのインジェクション、統計計算、および相関計算を行います。
- **カーネルバイパスの処理能力**：Polars と Rayon を活用した、ハードウェアリソースを極限まで飽和させるベクトル化された高速アナリティクス。
- **ソブリン（主権）インターフェース**：低レベルの IPC やメモリ管理の複雑さを隠蔽する、使いやすい Python API (`SovereignFrame`)。

## アーキテクチャ
Nairobi OS は、D-Bus と共有メモリを介して接続された 3 つの専門的なコンポーネントで構成されています：
1. **Nairobi Axum Refinery**：高性能な Rust コア。生のデータインジェクションと並列化されたアナリティクスを管理します。
2. **Nairobi Hub**：IPC オーケストレーター。refinery デーモンとクライアント間のファイル記述子と信号を調整します。
3. **Lagos Vision**：視覚皮層。`memfd` ハンドルを直接 GPU パイプラインにマップする、ヘッドレスなイベント駆動型レンダリングエンジン。
4. **Nairobi Connector**：セマンティックブリッジ。Linux デスクトップのアクセシビリティツリーを LLM に公開する MCP サーバー。
5. **Nairobi Python**：高レベルブリッジ。Rust エコシステム全体に対する Pythonic なインターフェースを提供します。

## インストール

### PyPI からのインストール
```bash
pip install nairobi-os
```

### ソースからのビルド
1. **リポジトリのクローン**:
    ```bash
    git clone https://github.com/KevinKenya/nairobi-connector-open-source.git
    cd nairobi-connector-open-source
    ```

2. **仮想環境の設定**:
    ```bash
    python3 -m venv .venv
    source .venv/bin/activate
    pip install maturin pyo3-build-config zbus anywidget traitlets pandas
    ```

3. **ワークスペースのビルド実行**:
    ```bash
    ./build_wheel.sh --release
    ```

## 使用方法

### データ分析 (インメモリパイプライン)
```python
import nairobi_os as nb

# バックグラウンドの refinery デーモンを起動して接続
nb.connect()

# ゼロコピーメモリパイプラインを使用してデータセットを読み込む
frame = nb.read_csv("simulator/fndds_ingredient_nutrient_value.csv")

# Rust refinery を介してベクトル化された計算を実行
profile = frame.crunch("value")
print(f"Mean: {profile['mean']:.4f}")
print(f"Std Dev: {profile['std_dev']:.4f}")

# メモリマップされた frame 上で任意の SQL クエリを直接実行
subset = frame.query("SELECT * FROM dataset WHERE value > 50.0")

# Lagos 加速インタラクティブプロッティングウィジェットを表示
subset.plot(column="value")
```

### コンピュータ使用 (MCP サーバー)
Nairobi Connector を使用する AI エージェントは、以下の基本的なループに従う必要があります：
1. `nairobi_find_window` を使用してターゲットのウィンドウを指定します。
2. `nairobi_get_ui_map` を介して現在の状態を確認します。
3. 目的のインタラクティブ要素の TOON `[ID: N]` を読み取ります。
4. `nairobi_interact` または `nairobi_type_text` を使用して、その要素に対してアクションを実行します。

## システム設定 (貢献者ガイド)

### ヒュージページ (Huge Pages)
Refinery エンジンは、ゼロコピーバッファのために 1GB ヒュージページを優先的に使用します。ホストマシンでこれらを有効にするには：
```bash
echo 1 | sudo tee /sys/kernel/mm/hugepages/hugepages-1048576kB/nr_hugepages
```
*注意: 1GB ページが利用できない場合、エンジンは自動的に透過的ヒュージページ (THP) にフォールバックします。*

### io_uring と SQPOLL
`DiracEngine` は、最大の I/O スループットを達成するために、`SQPOLL`（カーネルポーリングスレッド）付きの `io_uring` を使用します。

## サポート
Nairobi OS がお役に立ちましたら、ぜひ本プロジェクトの支援をご検討ください：

[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-PayPal-blue.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=chege.finance@gmail.com&item_name=Support+Nairobi+OS+Development)

## ライセンス
このプロジェクトは **Apache License 2.0** の下でライセンスされています。  
*(注意: TOON フォーマットおよびブリッジの実装の一部は TOON の著者に帰属します。)*

---
© 2026 Kevin Chege. All Rights Reserved.
