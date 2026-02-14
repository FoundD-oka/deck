
▼計画書: deck — Claude Code マルチセッションマネージャー（TUI）

▼1. スコープ定義

▼1-1. コンセプト

deck: 複数のClaude Codeセッションを1画面で統合管理するFrankenTUIアプリ。
計画作成は外部（テキストエディタ等）で行い、deckは「指示・監視・介入」に特化する。
usage-guideに定義されたスキルベースの自動化フローをPTY経由でそのまま実行する。

▼1-2. 今回の完成定義（MVPの合格ライン）
	1.	セッションを複数作成し、それぞれに指示を送れる。
	2.	全セッションの状態（実行中・待機・完了・エラー）がステータスバーで常時見える。
	3.	セッション切替でログ・ディレクトリ・ファイル表示が連動する。
	4.	PTYでClaude Codeを起動して指示を送り、ログが流れ、needs_inputで止まれる。
	5.	ディレクトリツリーからファイルをプレビューできる。エディタ起動でbr編集も可能。
	6.	ログと状態が保存され、アプリを閉じても続きから再開できる。

▼1-3. 今回はやらない（明確に除外）

・アプリ内でのプラン作成・編集UI（外部エディタに委ねる）
・アプリ内でのbrタスクのインライン編集UI（エディタ起動 or br CLIで対応）
・スケジュール自動実行
・自動応答による確認待ち突破（安全のため）
・Git GUI操作パネル（Claude Codeに指示するか、ターミナルで直接やる）

――――

▼2. 画面仕様（FrankenTUI パネル構成）

┌─────────────┬────────────────────────┬─────────────────┐
│ セッション    │ ファイル表示            │ ログ             │
│ 一覧         │                        │ (統合/個別切替)  │
│              │ 選択ファイルの内容表示   │                  │
│ [1] ● API    │ （閲覧専用）            │ session-1: ...   │
│ [2] ◆ Test   │                        │ session-2: ...   │
│ [3] ○ DB     │                        │ session-3: ...   │
│──────────────│                        │                  │
│ ディレクトリ  │                        │                  │
│ 📁 src/      │                        │                  │
│ 📁 .beads/   │                        │                  │
│ 📁 plans/    │                        │                  │
├─────────────┴────────────────────────┴─────────────────┤
│ [session-1] > タスクを実行して                           │
├─────────────────────────────────────────────────────────┤
│ 実行中: 2 │ 待機: 1 │ 完了: 3 │ エラー: 0 │ 入力待ち: 1 │
└─────────────────────────────────────────────────────────┘

▼2-1. 左上：セッション一覧

目的は「全セッションの状態を一覧で把握し、瞬時に切り替える」です。

・表示項目
	1.	セッション名（ユーザー命名 or 自動採番）
	2.	状態アイコン（● running / ◆ needs_input / ○ queued / ✓ done / ✗ failed）
	3.	作業ディレクトリ（短縮表示）
	4.	最終更新時刻
・操作
	5.	上下キーでセッション選択（ログ・ディレクトリ・ファイルが連動切替）
	6.	n: 新規セッション作成（作業ディレクトリを指定）
	7.	d: セッション削除（確認あり）
	8.	r: セッション名変更

▼2-2. 左下：ディレクトリツリー

目的は「選択中セッションの作業ディレクトリを探索する」です。

・表示内容
	1.	選択中セッションの root_path 配下のツリー
	2.	.beads/ も表示（brのタスクファイルにアクセスできる）
	3.	plans/ も表示（プランファイルを確認できる）
・操作
	4.	上下キー＋Enter でツリー展開・縮小
	5.	Enter でファイル選択 → 中央パネルにプレビュー表示
	6.	e: 選択ファイルを $EDITOR で開く（vim等でbr関連ファイルやプランを編集）
	7.	隠しファイル表示のトグル（h キー）

▼2-3. 中央：ファイル表示

目的は「選択したファイルの内容を素早く確認する」です。

・表示内容
	1.	ディレクトリツリーで選択したファイルの内容（閲覧専用）
	2.	シンタックスハイライト（対応可能な範囲で）
	3.	セッション切替時は前回選択ファイルを復元
・操作
	4.	スクロール（j/k or 上下キー）
	5.	e: $EDITOR で開く

▼2-4. 右：ログパネル

目的は「セッションの出力をリアルタイムで監視する」です。

・表示モード（キー1つで切替: Tab）
	1.	統合ログ: 全セッションの出力を時系列で表示（セッション名を色分けプレフィックス）
	2.	個別ログ: 選択中セッションの出力のみ表示
・表示内容
	3.	PTY出力をそのまま表示（ANSI色対応）
	4.	自動スクロール（最新出力を追跡）
	5.	手動スクロールで履歴確認（自動スクロールは一時停止）
・操作
	6.	Tab: 統合↔個別切替
	7.	/: ログ内検索

▼2-5. 下部：指示入力欄

目的は「選択中セッションのClaude Codeに指示を送る」です。

・表示
	1.	プロンプトに対象セッション名を表示（例: [session-1] > ）
	2.	needs_input 時にハイライト（入力を促す視覚的合図）
・操作
	3.	テキスト入力 → Enter で送信（PTYに書き込み）
	4.	Ctrl+C: 選択中セッションのプロセスに SIGINT 送信
	5.	上下キー: 入力履歴の呼び出し

▼2-6. 最下部：ステータスバー

目的は「全体状況を常に把握する」です。

・表示内容
	1.	状態別セッション数: 実行中: N │ 待機: N │ 完了: N │ エラー: N │ 入力待ち: N
	2.	キーバインドヘルプ（現在のパネルで使えるキー）
・更新タイミング
	3.	セッション状態が変わるたびに即時更新

――――

▼3. 状態設計

▼3-1. セッション状態（固定）
	1.	queued（未実行: PTY未起動）
	2.	running（実行中: PTYプロセスあり、出力流れ中）
	3.	needs_input（人の入力待ち: 停止兆候検知）
	4.	done（成功: exit_code 0）
	5.	failed（失敗: exit_code != 0）

▼3-2. 状態遷移
	1.	queued → running: PTY起動 + 指示送信
	2.	running → done: プロセス終了コードが0
	3.	running → failed: 終了コードが0以外
	4.	running → needs_input: 停止兆候を検知、またはユーザーが手動で切替
	5.	needs_input → running: 入力送信
	6.	failed → running: 再実行（同セッションでPTYを再起動）
	7.	done → queued: 次の指示を待つ状態にリセット

▼3-3. needs_inputの検知（MVPの方針）

・自動検知は"保守的"にします（誤検知で止まる方が安全）。
・初期は次のどれかでneeds_inputにします。
	1.	出力に確認っぽい文字が出た（例：y/n、confirm、continue）
	2.	一定時間（設定値）出力が止まった
	3.	ユーザーが手動でneeds_inputに切り替えた（キーバインド: m）
・自動で勝手に「y」などを返す機能は入れません。

▼3-4. brとの状態同期

セッション状態とbrタスク状態は別物として管理する。

・セッション状態: アプリが管理（PTYの生死に基づく）
・brタスク状態: Claude Codeが管理（br ready / br close で操作）
・アプリはbrをポーリング（3秒間隔）して、セッション一覧の補助情報として表示

brの状態はあくまで参考表示。アプリの動作判定には使わない。
理由: Claude Codeが1セッション内で複数タスクを連続実行することがあり、
brの状態とセッションの状態は1:1対応しないため。

――――

▼4. データモデル

▼4-0. 設計方針

・アプリは「複数PTYの管制塔」に特化する
・計画作成は外部（テキストエディタ）に委ねる
・brタスク管理はClaude Codeが行う。アプリはbrを読み取り表示はするが、書き込みはしない
・brタスクの編集が必要な場合は、ディレクトリツリーからエディタを起動するか、br CLIを直接使う
・アプリが所有するのはセッション情報とログのみ

▼4-1. Session

・id（UUID）
・name（ユーザー命名 or 自動: session-1, session-2...）
・root_path（作業ディレクトリ）
・status（queued / running / needs_input / done / failed）
・pty_pid（実行中のプロセスID。未起動ならnull）
・instruction（最後に送った指示文）
・log_path
・created_at / updated_at
・exit_code（任意）

▼4-2. AppConfig

・sessions_file_path（デフォルト: ~/.config/deck/sessions.json）
・logs_root_path（デフォルト: ~/.config/deck/logs/）
・needs_input_timeout_sec（デフォルト: 30）
・br_poll_interval_sec（デフォルト: 3）
・editor（デフォルト: $EDITOR or vim）

▼4-3. 保存形式

・セッション一覧: sessions.json（全セッションのメタデータ）
・ログ: {logs_root_path}/{session_id}.log（セッションごと1ファイル）
・brデータ: 各プロジェクトの .beads/beads.db（アプリは読むだけ、書かない）

――――

▼5. 実行設計（PTYでClaude Codeを動かす）

▼5-1. 実行単位の原則

・1セッション＝1PTYプロセス
・1つのセッションは1つの作業ディレクトリに紐づく
・同じディレクトリに複数セッションを紐づけてもよい（並列タスク実行）

▼5-2. 起動フロー

	1.	セッションのroot_pathを作業ディレクトリに設定
	2.	PTYを作成し、Claude Codeを起動（claude コマンド）
	3.	起動完了後、指示入力欄から送信された文字列をPTYに書き込み
	4.	出力をストリームで受け取り、ログパネルに表示 + ファイルに追記
	5.	終了検知 → status更新（done or failed）
	6.	needs_input検知 → statusをneeds_inputに更新、ステータスバーに反映

▼5-3. PTYに送信するコマンドのパターン

アプリが独自コマンドを持つわけではない。
ユーザーがusage-guideの呼び出し文をそのまま入力欄に打つ。

よく使うパターン:
・「タスクを実行して」 → Task Executor
・「このプランを計画して plans/xxx.md」 → Plan & Task Creator
・「/doc-update」 → doc-updateスキル
・「/doc-review」 → doc-reviewスキル
・「続けて」 → Task Executor（継続）

▼5-4. 複数セッション同時実行

・各セッションのPTYは独立したプロセス
・アプリのイベントループで全PTYの出力をノンブロッキングで監視
・FrankenTUIの差分レンダリングにより、複数ログが同時更新されても画面がちらつかない

▼5-5. ログの扱い

・画面に見せたものは、必ず同じ内容がファイルに残る
・セッションごとに1ログファイル
・ログ先頭にメタ情報（セッション名、作業ディレクトリ、開始時刻）を記録
・統合ログ表示はファイルには書かない（表示上のマージのみ）

――――

▼6. br連携設計（読み取り専用）

▼6-1. アプリとbrの関係

・アプリはbrのDBを**読み取りのみ**行う
・brへの書き込みはClaude Code（PTY経由）が行う
・ユーザーがbrタスクを直接編集したい場合:
  方法A: ディレクトリツリーで .beads/ を選択 → e キーでエディタ起動
  方法B: 別ターミナルで br update / br dep add 等を直接実行
  方法C: Claude Codeに「bd-3のタイトルを変えて」と指示

▼6-2. brの表示（補助情報）

セッション一覧にbrの進捗を補助表示する:

・セッション選択時、そのroot_pathの .beads/ が存在すれば:
  - br list --json の結果からopen/closed件数を取得
  - セッション名の横に「[3/8 tasks]」のように表示
  - br epic status --json からEpic進捗を取得可能

・ポーリング間隔: br_poll_interval_sec（デフォルト3秒）

▼6-3. 全体フロー

```
[外部エディタ]              [TUIアプリ]                     [PTY: Claude Code]       [br DB]
    │                          │                                │                     │
    │ plans/*.md を作成         │                                │                     │
    │                          │                                │                     │
    │                      セッション作成                        │                     │
    │                      root_path指定                        │                     │
    │                          │                                │                     │
    │                      指示入力:                             │                     │
    │                      「このプランを計画して               │                     │
    │                       plans/xxx.md」                      │                     │
    │                          ├──── PTY書き込み ──────────────→│                     │
    │                          │                                │── Plan & Task ─────→│
    │                          │                                │   Creator           │ br create
    │                          │                                │                     │
    │                          │←── ログ表示 ←─────────────────│                     │
    │                          │                                │                     │
    │                          │←── br list --json（ポーリング）──────────────────────←│
    │                          │    [3/8 tasks] 表示             │                     │
    │                          │                                │                     │
    │                      指示入力:                             │                     │
    │                      「タスクを実行して」                   │                     │
    │                          ├──── PTY書き込み ──────────────→│                     │
    │                          │                                │── Task Executor ───→│
    │                          │                                │   br ready          │ br close
    │                          │                                │   実装              │
    │                          │                                │   /doc-update       │
    │                          │                                │                     │
    │                          │  needs_input検知               │                     │
    │                          │  → ステータスバー更新           │                     │
    │                          │  → ユーザーが入力欄に回答       │                     │
    │                          ├──── PTY書き込み ──────────────→│                     │
    │                          │                                │                     │
    │  brタスク編集が必要な場合: │                                │                     │
    │  方法A: e キーでエディタ   │                                │                     │
    │  方法B: 別ターミナルで    │                                │                     │
    │    br update {id} --title│                                │                     │
    │                          │                                │                     │
```

――――

▼7. 技術選定: FrankenTUI

▼7-1. 選定理由

リポジトリ: https://github.com/Dicklesworthstone/frankentui
言語: Rust（nightly toolchain）
アーキテクチャ: Elm/Bubbletea 型（Model-Update-View）

本アプリの要件に対して、FrankenTUIの特性が直接的に合致する:

| 要件 | FrankenTUIの特性 | 効果 |
|------|-----------------|------|
| 複数ログ同時更新で画面が崩れない | 差分レンダリング（BufferDiff） | 変更セルだけ再描画。5セッション同時更新でもちらつかない |
| ログが流れてもUIパネルが安定 | インラインモード（ScreenMode::Inline） | UIは画面下部に固定、ログはスクロールバック領域に流れる |
| クラッシュしてもターミナルが壊れない | RAIIクリーンアップ（TerminalSession::Drop） | パニックしてもraw mode解除・カーソル復帰が保証される |
| 複数PTYのノンブロッキング監視 | Subscription + async Cmd | tick_every() + Cmd::perform() で全PTYを非同期監視 |
| PTYとの統合 | ftui-pty クレート | PTYテストユーティリティが最初から存在 |

▼7-2. 使用クレート構成

| クレート | 本アプリでの用途 |
|---------|-----------------|
| ftui | 公開ファサード。use ftui::prelude::* で一括インポート |
| ftui-core | ターミナルライフサイクル、キーイベント取得 |
| ftui-render | バッファ差分計算、ANSIプレゼンター |
| ftui-runtime | Elm型イベントループ（App, Model, Cmd, Subscription） |
| ftui-widgets | List, Tree, Paragraph, Input, Block, Tabs, Progress |
| ftui-layout | Flex/Grid ソルバー（Layout::horizontal, Layout::vertical） |
| ftui-style | スタイル・テーマ（セッション状態の色分け等） |
| ftui-text | Spans, Segments（ログの色分けプレフィックス等） |
| ftui-pty | PTYプロセス管理のユーティリティ |

▼7-3. アーキテクチャ: Model-Update-View

FrankenTUIのElm型ループがそのままアプリの設計になる:

```
Event（キー入力, PTY出力, tick, brポーリング結果）
  ↓
Message（型安全なenumに変換）
  ↓
update()（状態を更新、副作用をCmdで返す）
  ↓
view()（状態からUIを描画。immutable borrow）
  ↓
Frame → Buffer → Diff → ANSI → stdout
```

▼7-3-1. Model（アプリ状態）

```rust
struct AppState {
    sessions: Vec<Session>,
    active_session: usize,
    active_panel: Panel,
    log_mode: LogMode,           // Unified / Individual
    dir_tree: TreeState,
    file_preview: Option<String>,
    input_history: Vec<String>,
    br_tasks: HashMap<PathBuf, Vec<BrTask>>,  // root_path → タスク一覧
}

enum Panel { SessionList, DirTree, FilePreview, Log, Input }
enum LogMode { Unified, Individual }
```

▼7-3-2. Message（イベント型）

```rust
enum Msg {
    // キー入力
    Key(KeyEvent),

    // セッション操作
    SelectSession(usize),
    CreateSession { name: String, root_path: PathBuf },
    DeleteSession(usize),
    RenameSession(usize, String),

    // PTY
    PtyOutput { session: usize, data: Vec<u8> },
    PtyExited { session: usize, exit_code: i32 },
    SendInput(String),
    SendSigint,

    // 状態遷移
    NeedsInputDetected(usize),
    ManualNeedsInput,

    // UI
    ToggleLogMode,
    SwitchPanel(Panel),
    TreeToggle(PathBuf),
    PreviewFile(PathBuf),
    OpenEditor(PathBuf),
    LogSearch(String),

    // br（補助表示）
    BrPollTick,
    BrPollResult { root_path: PathBuf, tasks: Vec<BrTask> },

    // システム
    Tick,
    Quit,
}
```

▼7-3-3. Subscription（常時監視）

```rust
fn subscriptions(&self) -> Vec<Box<dyn Subscription<Msg>>> {
    let mut subs = vec![
        tick_every(Duration::from_millis(100)),  // needs_inputタイムアウト監視
    ];
    // 各セッションのPTY出力を監視
    for (i, session) in self.sessions.iter().enumerate() {
        if session.status == Status::Running {
            subs.push(pty_output_watcher(i, &session.pty));
        }
    }
    // brポーリング
    subs.push(tick_every(Duration::from_secs(self.config.br_poll_interval_sec)));
    subs
}
```

▼7-4. パネルレイアウト構成（実装イメージ）

```rust
fn view(&self, frame: &mut Frame) {
    // 最上位: 本体 / 入力欄 / ステータスバー
    let [body, input_area, status_bar] = Layout::vertical([
        Constraint::Min(10),
        Constraint::Length(3),
        Constraint::Length(1),
    ]).areas(frame.area());

    // 本体: 左サイドバー / 中央 / 右ログ
    let [sidebar, center, log_panel] = Layout::horizontal([
        Constraint::Length(28),
        Constraint::Percentage(40),
        Constraint::Min(30),
    ]).areas(body);

    // 左サイドバー: セッション一覧 / ディレクトリツリー
    let [session_list, dir_tree] = Layout::vertical([
        Constraint::Percentage(40),
        Constraint::Percentage(60),
    ]).areas(sidebar);

    // 各パネルをレンダリング
    self.render_session_list(frame, session_list);
    self.render_dir_tree(frame, dir_tree);
    self.render_file_preview(frame, center);
    self.render_log(frame, log_panel);
    self.render_input(frame, input_area);
    self.render_status_bar(frame, status_bar);
}
```

▼7-5. ウィジェットとプラン要素の対応

| パネル | ウィジェット | 活用する機能 |
|--------|------------|------------|
| セッション一覧 | List | 仮想化スクロール、選択ハイライト、カスタムアイテム描画（状態アイコン） |
| ディレクトリツリー | Tree | expand/collapse、ノード選択 |
| ファイル表示 | Paragraph or Textarea | ワードラップ、スクロール、（Textareaならシンタックスハイライトフック） |
| ログ | Paragraph | 自動スクロール、ANSIカラー対応、手動スクロール時の自動追跡停止 |
| 指示入力 | Input | カーソル操作、選択、**入力履歴** |
| ステータスバー | Paragraph + Spans | セッション状態別の色分けカウント |
| 各パネル枠 | Block | ボーダー、タイトル、フォーカス時のハイライト |
| br進捗 | Progress | Epic進捗バー（補助表示） |

▼7-6. 注意点

・FrankenTUIは初期段階のプロジェクト（star 155、2026年2月時点）
・Rust nightlyツールチェーンが必要
・Ratatuiのようなエコシステムの厚みはまだない
・足りないウィジェットや機能は自作が必要な場合がある
・ただしアーキテクチャが堅牢（unsafe禁止、循環依存なし、レイヤード設計）なので拡張しやすい

――――

▼8. 開発マイルストーン

▼フェーズA: 基盤（パネルレイアウト + セッション管理）

使用: ftui-runtime, ftui-layout, ftui-widgets(List, Block, Paragraph, Input)

	1.	Rust nightly + FrankenTUI ワークスペースセットアップ
	2.	Model/Msg/update/view の骨格実装
	3.	Layout::horizontal + vertical でパネル6分割
	4.	Block でパネル枠描画、フォーカスハイライト
	5.	List でセッション一覧（作成・削除・切替）
	6.	Paragraph でステータスバー（状態集計）
	7.	sessions.json の保存・復元
	8.	キーバインド: パネル間移動（Ctrl+h/j/k/l）、セッション選択（↑↓）、n/d/r

▼フェーズB: PTY実行（コア機能）

使用: ftui-pty, ftui-runtime(Cmd::perform, Subscription)

	1.	Session構造体にPTYプロセスを持たせる
	2.	セッション選択 → PTYでclaude起動（Cmd::perform で非同期）
	3.	Subscription で各PTYの出力を監視 → PtyOutput メッセージ
	4.	Input ウィジェットで指示入力 → PTYに書き込み
	5.	PtyExited メッセージで done/failed 状態更新
	6.	Ctrl+C で選択セッションに SIGINT 送信
	7.	複数セッション同時実行のテスト（3セッション並行）

▼フェーズC: 監視・介入

使用: ftui-runtime(Subscription::tick_every), ftui-text(Spans)

	1.	PTY出力のパターンマッチ（y/n, confirm等）→ NeedsInputDetected
	2.	tick_everyでタイムアウト監視（設定秒数出力なし → needs_input）
	3.	m キーで手動 needs_input 切替
	4.	ログ表示: Paragraph に統合ログ / 個別ログの切替（Tab）
	5.	統合ログでセッション名を色分けプレフィックス（Spans）
	6.	/ キーでログ内テキスト検索
	7.	ログのファイル保存（セッションごと1ファイル）

▼フェーズD: ディレクトリ・ファイル探索

使用: ftui-widgets(Tree, Paragraph or Textarea)

	1.	Tree ウィジェットでディレクトリツリー表示
	2.	セッション切替でツリーのルートを連動変更
	3.	Enter でファイル選択 → 中央パネルに Paragraph でプレビュー
	4.	e キーで $EDITOR をサブプロセス起動（TUIを一時サスペンド）
	5.	h キーで隠しファイル表示トグル
	6.	.beads/ と plans/ の存在を視覚的にハイライト

▼フェーズE: br連携（補助表示）

使用: ftui-widgets(Progress), ftui-runtime(Cmd::perform)

	1.	セッションのroot_pathに .beads/ が存在するか検出
	2.	Cmd::perform で br list --json を非同期実行
	3.	セッション一覧に [3/8 tasks] のタスク件数を補助表示
	4.	br epic status --json から Progress ウィジェットで進捗バー
	5.	ポーリング間隔: 設定値（デフォルト3秒）

▼フェーズF: 仕上げ

	1.	セッション状態の永続化（sessions.json にstatus, last_instruction保存）
	2.	アプリ再起動時のセッション復元（PTYは再接続できないため queued にリセット）
	3.	Input ウィジェットの入力履歴（上下キー）
	4.	needs_input タイムアウトの設定（AppConfig）
	5.	例外処理: claude コマンド未インストール、.beads/ なし、PTY起動失敗
	6.	ScreenMode::AltScreen / Inline の切替オプション

――――

▼9. リスク管理
	1.	PTYが詰まる、読み取りが止まる
		対策: 出力停止タイムアウト、手動needs_input切替、停止ボタン
	2.	AIが破壊的コマンドを提案する
		対策: 自動応答しない。needs_inputで止めて人が判断
	3.	複数セッションが同じファイルを同時編集してコンフリクト
		対策: MVPでは「1ディレクトリ=1セッション」を推奨。警告表示
	4.	brのDB同時アクセス
		対策: アプリは読み取り専用。書き込みはClaude Code or br CLI のみ。SQLiteのWALモードで読み取り競合なし

――――

▼10. まとめ

deckは「複数のClaude Codeセッションを1画面で統合管理するTUI」です。

やること:
・複数PTYの同時起動・監視・介入
・ログの統合/個別表示
・ディレクトリ探索とファイルプレビュー
・brタスク進捗の補助表示

やらないこと（外部に委ねる）:
・計画作成（テキストエディタ）
・brタスク編集（br CLI or エディタ）
・Git操作（Claude Codeへの指示 or ターミナル直接）

完成度を決めるのは:
	1.	needs_inputで止まれること（安全性）
	2.	全セッションの状態がステータスバーで常時見えること（俯瞰性）
	3.	ログが残ること（再現性）

この3点が最初から固定されているため、運用段階で詰まりにくい設計になっています。
