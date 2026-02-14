---
owner: プロダクトオーナー
updated: 2026-02-14
watch_paths: []
staleness_days: 90
breaks_build_if_stale: false
---

# Intent

## What We Are Building
deck: 複数の Claude Code セッションを1つのターミナル画面で統合管理する TUI アプリケーション。セッションごとに PTY を起動し、指示送信・出力監視・入力待ち検知を行う。Rust nightly + FrankenTUI v0.1.1 で構築。

## What We Are NOT Building
- Claude Code 自体の代替や拡張（deck は既存の claude CLI をラップするだけ）
- Web UI やデスクトップ GUI（ターミナル TUI のみ）
- 自動応答機能（入力待ちを検知するが、応答は人間が行う）
- リモートセッション管理（ローカルマシン上の PTY のみ）
- ファイル編集機能（プレビューのみ。編集は外部エディタに委譲）

## Current Phase
MVP 完了: Phase A〜F の全6フェーズを実装済み。セッション管理・PTY実行・入力待ち検知・ディレクトリ探索・br連携・入力履歴・エラー処理が動作する。

## Future Direction
- ログ内テキスト検索（`/` キー）
- PTY リサイズ対応（ターミナルサイズ変更時の追従）
- セッション間のコピー＆ペースト
- カスタムキーバインド設定

## Success Criteria
- セッションを3つ以上同時作成し、それぞれに独立した指示を送信できる
- 全セッションの状態がステータスバーに表示され、1秒以内に反映される
- セッション切替でログ・ツリー・プレビューが連動する
- Claude Code の確認待ちを検知し、入力待ち状態に遷移する（自動応答なし）
- 終了→再起動でセッション一覧+ログが復元される（PTY は Queued にリセット）
