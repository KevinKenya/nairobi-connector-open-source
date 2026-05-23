[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [Русский](README.ru.md) | [한국어](README.ko.md)

# Nairobi Axum Refinery

## 概要
**Axum Refinery** は、Nairobi OS の超高性能コアです。Rust で書かれており、カーネルバイパス I/O とベクトル化された並列アナリティクスを介して、現代のハードウェアリソースを極限まで飽和させるように設計されています。匿名メモリファイル記述子 (`memfd`) にロードされたデータのライフサイクルを管理する D-Bus サービスとして機能します。

## 主要な機能
- **Dirac Ingestion Engine**：`io_uring` (Tier 1)、`copy_file_range` (Tier 2)、および `mmap` (Tier 3) を活用した 3 層のデータ接入戦略。
- **Axiom Crunch**：Polars と Rayon による、ベクトル化された統計モーメント計算（平均、分散、歪度、尖度）。
- **Relational Strike**：最適化されたピアソンおよびスピアマンの相関係数計算。
- **SQL Analytics**：`polars-sql` を使用した、メモリ常駐データに対する SQL クエリの直接実行。
- **ゼロコピーデータプレーン**：`iceoryx2` 共有メモリおよび D-Bus を介して分析結果を公開します。

## アーキテクチャ
Refinery は、いくつかの専門化されたエンジンで構成されています：
- `DiracEngine`：ハードウェア加速された I/O を処理します。
- `AnalyzeEngine`：統計計算と SQL 実行を実行します。
- `DbusService`：`org.nairobi.NairobiAxumRefinery1` インターフェースを実装します。

## インストールとセットアップ

### 前提条件
- **カーネル**：Linux 5.10+ (WSL2 をサポート)。
- **依存関係**：`libdbus-1-dev`, `pkg-config`。
- **ヒュージページ (Huge Pages)**：1GB ヒュージページを有効にすると、エンジンは最高のパフォーマンスを発揮します。
    ```bash
    echo 1 | sudo tee /sys/kernel/mm/hugepages/hugepages-1048576kB/nr_hugepages
    ```

### ビルド
```bash
cargo build --release -p nairobi-axum-refinery
```

## 開発ガイド

### カーネルレベルの設定
`DiracEngine` は `IORING_SETUP_SQPOLL` の使用を試みます。root 権限なしでこれを動作させるには、`/proc/sys/kernel/unprivileged_userns_clone` を調整するか、`CAP_SYS_ADMIN` を付与して実行する必要がある場合があります。

### チュートリアル：新しい統計指標の追加
1.  **指標の定義**：`src/analyze.rs` 内で `StatisticalProfile` 構造体とその `compute` メソッドを更新します。
2.  **プロトコルの更新**：`crates/nairobi-protocol/src/types.rs` 内の `DistilledAnalytics` 構造体に新しいフィールドを追加します。
3.  **D-Bus 経由での公開**：`src/dbus_service.rs` 内の D-Bus インターフェースが、更新されたプロファイルを正しくシリアライズすることを確認します。

### テスト
Refinery は非同期統合テストのために `tokio::test` を使用します。
```bash
cargo test -p nairobi-axum-refinery
```

#### 単体テスト用のモック処理
D-Bus レイヤをバイパスして、手動で `memfd` を作成して `AnalyzeEngine` を単独でテストできます：
```rust
let opts = memfd::MemfdOptions::default();
let mfd = opts.create("test.csv")?;
// テストデータの書き込み...
let engine = AnalyzeEngine::new()?;
let results = engine.analyze(mfd.into_fd(), "target_column")?;
```

## トラブルシューティング
- **`io_uring` の初期化失敗**：カーネルが `io_uring` をサポートしているか確認してください (`zgrep CONFIG_IO_URING /proc/config.gz`)。
- **ヒュージページの割り当て失敗**：ホストに十分な連続メモリがあるか確認してください (`grep Huge /proc/meminfo`)。

## サポート
Nairobi OS がお役に立ちましたら、ぜひ本プロジェクトの支援をご検討ください：

[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-PayPal-blue.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=chege.finance@gmail.com&item_name=Support+Nairobi+OS+Development)

## ライセンス
このプロジェクトは **Apache License 2.0** の下でライセンスされています。

---
© 2026 Kevin Chege. All Rights Reserved.
