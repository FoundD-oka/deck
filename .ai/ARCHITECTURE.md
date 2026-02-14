---
owner: テックリード
updated: 2026-02-14
watch_paths:
  - src/
  - lib/
  - packages/
staleness_days: 30
breaks_build_if_stale: false
---

# Architecture

## Module Map

| Module | Path | Responsibility | Owns (data/API) | Depends On |
|--------|------|----------------|------------------|------------|
| app | `src/app.rs` | Elm MVU: AppState, Msg, update(), view() | AppState, Panel, LogMode | config, session, log_store, pty_manager, needs_input, dir_tree, file_preview, input_history, br_poller, persistence, ui |
| session | `src/session.rs` | Session構造体、SessionStatus状態マシン | Session, SessionStatus | uuid, chrono, serde |
| pty_manager | `src/pty_manager.rs` | PTYライフサイクル管理（spawn/read/write/signal/exit） | PtyHandle | portable-pty |
| log_store | `src/log_store.rs` | セッション別ログバッファ（部分行結合、上限管理） | LogStore | uuid |
| needs_input | `src/needs_input.rs` | パターンマッチ + タイムアウトによる入力待ち検知 | NeedsInputDetector | regex |
| dir_tree | `src/dir_tree.rs` | ファイルシステムツリー（遅延展開、カーソル、隠しファイルトグル） | DirTree, FlatEntry | std::fs |
| file_preview | `src/file_preview.rs` | ファイル内容読み込み（1MB上限、バイナリ検知、スクロール） | FilePreview | std::fs |
| input_history | `src/input_history.rs` | 入力履歴（Vec + カーソル） | InputHistory | — |
| br_poller | `src/br_poller.rs` | `br list --json --all` 実行 + パース | BrTaskInfo | serde_json |
| config | `src/config.rs` | アプリ設定（パス、タイムアウト、エディタ） | AppConfig | dirs, serde |
| persistence | `src/persistence.rs` | sessions.json / ログファイルI/O | — | session, serde_json |
| ui::session_list | `src/ui/session_list.rs` | セッション一覧表示（状態アイコン、br件数） | — | app |
| ui::dir_tree_panel | `src/ui/dir_tree_panel.rs` | ツリーウィジェット描画 | — | app |
| ui::file_panel | `src/ui/file_panel.rs` | ファイルプレビュー描画 | — | app |
| ui::log_panel | `src/ui/log_panel.rs` | ログ表示（Individual/Unified） | — | app |
| ui::input_bar | `src/ui/input_bar.rs` | 入力バー描画 | — | app |
| ui::status_bar | `src/ui/status_bar.rs` | 状態集計バー描画 | — | app |

## Boundary Rules
- モジュール間通信は公開インターフェースのみ
- 循環依存は禁止
- コアロジック（session, pty_manager, needs_input, log_store, persistence）はFrankenTUI非依存
- UI層（app, ui/）のみがFrankenTUIに依存

## Entry Points
- Application: `src/main.rs` → `App::new(AppState).run()`
- Tests: `cargo test`

## Concurrency Boundaries
- Safe to work in parallel: 各module内部の変更、ui/ パネル個別変更
- Conflict-prone: app.rs（全モジュールを統合）、Msg enum、AppState struct
