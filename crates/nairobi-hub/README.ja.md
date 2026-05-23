[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [Русский](README.ru.md) | [한국어](README.ko.md)

# Nairobi Hub

## 概要
**Nairobi Hub** は、Nairobi OS の中央 IPC (Inter-Process Communication、プロセス間通信) オーケストレーターです。高性能な Rust 製 refinery デーモンとそのクライアントとの間で、ファイル記述子、D-Bus 信号、および共有メモリセグメントの調整を管理します。

## 主な機能
- **ファイル記述子 (FD) のプロキシ処理**：GVariant シグネチャを使用して、D-Bus 経由で `memfd` ファイル記述子を安全に渡します。
- **サービス管理**：`org.nairobi.NairobiAxumRefinery1` のライフサイクルを監視および管理します。
- **ハイブリッドデータプレーン**：パフォーマンス向上のために `iceoryx2` 共有メモリ経由で、または互換性向上のために D-Bus 経由で、データを動的にルーティングします。
- **セマンティックデコード**：生のバイナリアナリティクスを人間が読めるレポートやネイティブの Python 構造体にデコードします。

## アーキテクチャ
Nairobi Hub はいくつかの内部モジュールに分割されています：
- `client.rs`：D-Bus プロキシクライアント。
- `shm_subscriber.rs`：`iceoryx2` 共有メモリのサブスクリプションを処理します。
- `decoder.rs`：GVariant の結果を Markdown や JSON に変換します。

## 使用方法
Hub は主に、`nairobi-python` が refinery と通信するためのライブラリとして使用されます。

## 開発上の注意
Hub を変更する場合は、D-Bus インターフェースへの変更が `nairobi-protocol` にも反映されていることを確認してください。

## テスト
Hub の統合テストは、完全な IPC ラウンドトリップを検証します：
```bash
cargo test -p nairobi-hub
```

## サポート
Nairobi OS がお役に立ちましたら、ぜひ本プロジェクトの支援をご検討ください：

[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-PayPal-blue.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=chege.finance@gmail.com&item_name=Support+Nairobi+OS+Development)

## ライセンス
このプロジェクトは **Apache License 2.0** の下でライセンスされています。

---
© 2026 Kevin Chege. All Rights Reserved.
