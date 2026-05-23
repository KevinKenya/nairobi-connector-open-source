[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [Русский](README.ru.md) | [한국어](README.ko.md)

# Lagos Vision (lagos-lite)

## 概要
**Lagos Vision** は、Nairobi OS 用の超高性能レンダリングエンジンです。分析データを `memfd` 記述子経由で GPU パイプラインに直接メモリマッピングすることで、数百万のデータポイントをサブミリ秒のレイテンシで視覚化するように設計されています。Lagos は、WebSockets を介して Jupyter ノートブックのウィジェットに JPEG エンコードされたフレームをストリーミングする、ヘッドレスなデーモンとして動作します。

## 主要な機能
- **ゼロコピーレンダリング**：データは `memfd` ハンドルから `wgpu` バッファに直接メモリマップされます。
- **ハードウェア加速**：高性能なプロッティングのために `egui` と `wgpu`（Vulkan、Metal、DX12、または OpenGL）を使用します。
- **LTTB ダウンサンプリング**：大規模なデータセットの視覚的精度を維持するため、GPU 上で LTTB（Largest-Triangle-Three-Buckets）ダウンサンプリングアルゴリズムを実行します。
- **イベント駆動型アーキテクチャ**：アイドル時には CPU をまったく消費しません。データの更新やユーザーインタラクションがあった場合にのみレンダリングを行います。

## アーキテクチャ
Lagos Vision は、以下の要素で構成されています：
- **Lagos Lite**：レンダリングパイプラインを提供するコアライブラリ。
- **Lagos Vision Daemon**：`wgpu` サーフェスと WebSocket サーバーを管理するバイナリプロセス。
- **Lagos Widget**：ストリームを表示する Python 側の `anywidget` コンポーネント。

## インストールとセットアップ

### 前提条件
- **GPU**：Vulkan 互換の GPU（物理ディスプレイドライバがない場合は、ソフトウェアフォールバック用の OSMesa）。
- **システムライブラリ**：`libosmesa6-dev`, `mesa-utils`, `xvfb`。

### ビルド
```bash
cargo build --release -p lagos-lite --bin lagos-vision-daemon
```

## 使用方法

### Nairobi OS 内での使用
Lagos は通常、Python 側の `SovereignFrame.plot()` メソッドを介して透過的に使用されます。

### 手動でのデバッグ
レンダリングパイプラインをテストするために、デーモンを手動で起動できます：
```bash
./target/release/lagos-vision-daemon --fd <FD_INT> --width 1000 --height 400
```

## 開発ガイド

### カスタム可視化レイヤの実装
1.  **パイプラインの変更**：`src/pipeline.rs` 内で、カスタムの頂点シェーダーおよびフラグメントシェーダー (WGSL) を定義します。
2.  **バッファレイアウトの更新**：入力された `memfd` データを、新しいシェーダーのバインドグループにマッピングします。
3.  **UI への統合**：`src/device.rs` 内の `egui` インターフェースに、操作用コントロール（スライダー、ボタンなど）を追加します。

### ヘッドレス環境での運用
Google Colab などの物理ディスプレイが存在しないクラウド環境では、Lagos は `xvfb-run` や OSMesa を利用して動作します：
```bash
xvfb-run -s "-screen 0 1024x768x24" ./target/release/lagos-vision-daemon ...
```

## テスト
Lagos には、描画フレームをキャプチャしてゴールデンイメージと比較する、ビジュアル統合テストが含まれています。
```bash
cargo test -p lagos-lite
```

## トラブルシューティング
- **WebSocket 接続の失敗**：Colab や SageMaker などのクラウド環境では、プロキシポートが正しくマッピングされているか確認してください。Google Colab 環境を検出した場合、Nairobi Python がこれを自動的に処理します。
- **WGPU アダプターの未検出**：GPU ドライバが正しくインストールされているか確認してください。CPU のみの環境である場合、Lagos は自動的にソフトウェアレンダリング用のアダプターへのフォールバックを試みます。

## サポート
Nairobi OS がお役に立ちましたら、ぜひ本プロジェクトの支援をご検討ください：

[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-PayPal-blue.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=chege.finance@gmail.com&item_name=Support+Nairobi+OS+Development)

## ライセンス
このプロジェクトは **Apache License 2.0** の下でライセンスされています。

---
© 2026 Kevin Chege. All Rights Reserved.
