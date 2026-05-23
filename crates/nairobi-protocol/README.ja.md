[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [Русский](README.ru.md) | [한국어](README.ko.md)

# Nairobi Protocol

## 概要
**Nairobi Protocol** crate は、Nairobi OS エコシステム全体で使用される共有 D-Bus インターフェース、GVariant シグネチャ、およびデータ構造を定義します。これは Rust コア、Hub オーケストレーター、および Python バインディングにわたる型安全性の「信頼できる唯一の情報源 (Source of Truth)」として機能します。

## 主要なコンポーネント
- **インターフェース定義**：サービス名、オブジェクトパス、およびメソッドのシグネチャの定数。
- **共有型**：`DistilledAnalytics` や `CorrelationResult` などの GVariant 互換構造体。
- **メモリ管理**：`memfd` 操作用の `MemoryPipe` ラッパーと、`iceoryx2` アリーナの定義。

## D-Bus インターフェース
- **サービス名**：`org.nairobi.NairobiAxumRefinery1`
- **オブジェクトパス**：`/org/nairobi/NairobiAxumRefinery1`
- **インターフェース**：`org.nairobi.NairobiAxumRefinery1`

## 使用方法
Nairobi OS エコシステム内で通信する必要がある任意のコンポーネントの依存関係として、この crate を追加します。

## 開発上の注意
この crate への変更は細心の注意を払って行う必要があります。変更を加えると、依存するすべての crate を再コンパイルする必要があり、refinery と Python バインディングの間のバイナリ互換性が失われる可能性があります。

## テスト
統合テストは、GVariant シグネチャが想定される D-Bus プロトコルと一致することを確認します：
```bash
cargo test -p nairobi-protocol
```

## サポート
Nairobi OS がお役に立ちましたら、ぜひ本プロジェクトの支援をご検討ください：

[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-PayPal-blue.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=chege.finance@gmail.com&item_name=Support+Nairobi+OS+Development)

## ライセンス
このプロジェクトは **Apache License 2.0** の下でライセンスされています。

---
© 2026 Kevin Chege. All Rights Reserved.
