use crate::app::{AppState, Panel};
use crate::session::SessionStatus;
use ftui_core::geometry::Rect;
use ftui_render::cell::PackedRgba;
use ftui_render::frame::Frame;
use ftui_style::Style;
use ftui_widgets::paragraph::Paragraph;
use ftui_widgets::Widget;

fn panel_hints(panel: Panel) -> &'static str {
    match panel {
        Panel::SessionList => "↑↓:選択 n:新規 d:削除 r:名変 m:入力切替",
        Panel::DirTree => "↑↓:移動 Enter:開く h:隠しファイル",
        Panel::FilePreview => "↑↓:スクロール e:エディタで開く",
        Panel::Log => "t:個別/統合切替",
        Panel::Input => "Enter:送信 ↑↓:履歴 Esc:戻る",
    }
}

pub fn render(state: &AppState, frame: &mut Frame, area: Rect) {
    let mut running = 0u32;
    let mut queued = 0u32;
    let mut done = 0u32;
    let mut failed = 0u32;
    let mut needs_input = 0u32;

    for s in &state.sessions {
        match s.status {
            SessionStatus::Running => running += 1,
            SessionStatus::Queued => queued += 1,
            SessionStatus::Done => done += 1,
            SessionStatus::Failed => failed += 1,
            SessionStatus::NeedsInput => needs_input += 1,
        }
    }

    let hints = panel_hints(state.active_panel);
    let text = format!(
        " 実行中:{} | 待機:{} | 完了:{} | 失敗:{} | 入力待ち:{} | {} | Tab:移動 q:終了",
        running, queued, done, failed, needs_input, hints
    );

    let paragraph = Paragraph::new(text)
        .style(Style::new().fg(PackedRgba::rgb(229, 229, 229)).bg(PackedRgba::rgb(80, 80, 80)));
    paragraph.render(area, frame);
}
