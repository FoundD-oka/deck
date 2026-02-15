use crate::app::{AppState, Panel};
use crate::session::SessionStatus;
use crate::ui::theme;
use ftui_core::geometry::Rect;
use ftui_render::frame::Frame;
use ftui_style::Style;
use ftui_text::{Span, Text};
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
    let hint_style = Style::new().fg(theme::HINT_FG);
    let dim_style = Style::new().fg(theme::HINT_FG).dim();

    let spans = vec![
        Span::raw(" "),
        Span::styled(
            format!("実行中:{}", running),
            Style::new().fg(theme::STATUS_RUNNING),
        ),
        Span::raw(" | "),
        Span::styled(
            format!("待機:{}", queued),
            Style::new().fg(theme::STATUS_QUEUED),
        ),
        Span::raw(" | "),
        Span::styled(
            format!("完了:{}", done),
            Style::new().fg(theme::STATUS_DONE),
        ),
        Span::raw(" | "),
        Span::styled(
            format!("失敗:{}", failed),
            Style::new().fg(theme::STATUS_FAILED),
        ),
        Span::raw(" | "),
        Span::styled(
            format!("入力待ち:{}", needs_input),
            Style::new().fg(theme::STATUS_NEEDS_INPUT),
        ),
        Span::raw("  "),
        Span::styled(hints, hint_style),
        Span::raw("  "),
        Span::styled("Tab:移動 q:終了", dim_style),
    ];

    let text = Text::from_spans(spans);
    let paragraph = Paragraph::new(text).style(Style::new().bg(theme::BAR_BG));
    paragraph.render(area, frame);
}
