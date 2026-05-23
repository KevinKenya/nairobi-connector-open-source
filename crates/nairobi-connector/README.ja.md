[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [Русский](README.ru.md) | [한국어](README.ko.md)

# Nairobi Connector

## 概要
**Nairobi Connector** は、Nairobi OS 用の AT-SPI2 セマンティックブリッジおよび Model Context Protocol (MCP) サーバーです。Linux デスクトップのアクセシビリティツリーを、TOON (Token-Oriented Object Notation) と呼ばれる超高密度でトークン最適化されたフォーマットに変換して LLM や AI エージェントに公開することで、**「ピクセルなしのコンピュータ使用」**を実現します。ピクセルではなく純粋にセマンティックな UI ノードに基づいて操作するため、ほぼ瞬時のアクション実行を達成し、トークンのオーバーヘッドを劇的に削減します。

## 主な機能
- **ピクセルなしのコンピュータ使用**：AT-SPI2 を介して Linux デスクトップと直接相互作用し、スクリーンショット、OCR、または視覚的処理の必要性を排除します。
- **TOON 圧縮アルゴリズム**：生の D-Bus アクセシビリティツリーを高度に圧縮された Markdown 表現に変換します。非インタラクティブな「ノイズ」ノードを除外し、アクション可能な要素に連続する ID を割り当てることで、50ms 未満の処理速度で 500 トークン未満の出力を実現します。
- **MCP サーバー統合**：互換性のある LLM エージェントに対してセマンティックツールをネイティブに公開する、堅牢な `rmcp` ベースのサーバーを実装しています。
- **安全なセッションライフサイクル**：ハートビート監視機能を備え、`stdio` パイプがハングした場合に `RegistryLock` を自動的に解放して OS のハングを防止します。

## アーキテクチャ
Nairobi Connector は、LLM（MCP 経由）と Linux デスクトップ（AT-SPI2/D-Bus 経由）の間の双方向ブリッジとして機能します。これは `NeuralSession` レイヤをカプセル化し、ウィンドウの検出、UI ツリーのトラバース、および局所的なアクションインジェクションを管理します。

### 提供される MCP ツール
- `nairobi_find_window`：タイトル部分文字列（大文字小文字を区別しない）でウィンドウを検索し、ターゲットに指定します。
- `nairobi_get_ui_map`：現在の UI アクセシビリティツリーを TOON 圧縮されたマップとして返します。ターゲット指定用の連続する `[ID: N]` タグを持つインタラクティブ要素（ボタン、入力フィールド、チェックボックスなど）の高密度なリストを生成します。
- `nairobi_interact`：TOON ノード ID を使用して、UI 要素に対してセマンティックアクション（`click`、`activate`、`focus`）を実行します。
- `nairobi_type_text`：TOON ノード ID で指定された編集可能フィールド（Entry、TextArea など）にテキストをアトミックに挿入します。

## 使用方法
Nairobi Connector を使用するエージェントは、以下の基本的なループに従う必要があります：
1. `nairobi_find_window` を使用してウィンドウを指定します。
2. `nairobi_get_ui_map` を介して現在の状態を確認します。
3. 目的のインタラクティブ要素の TOON `[ID: N]` を読み取ります。
4. `nairobi_interact` または `nairobi_type_text` を使用して、その要素に対してアクションを実行します。
5. 再び相互作用する前に、ステップ 2 から繰り返して最新の ID を取得します。

## サポート
Nairobi OS がお役に立ちましたら、ぜひ本プロジェクトの支援をご検討ください：

[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-PayPal-blue.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=chege.finance@gmail.com&item_name=Support+Nairobi+OS+Development)

## ライセンス
このプロジェクトは **Apache License 2.0** の下でライセンスされています。  
*(注意: TOON フォーマットおよびブリッジの実装の一部は TOON の著者に帰属します。)*
