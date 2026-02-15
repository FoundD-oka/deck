use crate::app::{AppState, LogMode};
use crate::ui::theme;
use ftui_core::geometry::Rect;
use ftui_render::frame::Frame;
use ftui_widgets::paragraph::Paragraph;
use ftui_widgets::Widget;

pub fn render(state: &AppState, frame: &mut Frame, area: Rect, focused: bool) {
    let title = match state.log_mode {
        LogMode::Individual => "Log [individual] (t:toggle)",
        LogMode::Unified => "Log [unified] (t:toggle)",
    };

    let visible_height = area.height.saturating_sub(2) as usize;

    let content = match state.log_mode {
        LogMode::Individual => render_individual(state, visible_height),
        LogMode::Unified => render_unified(state, visible_height),
    };

    let is_empty = content.starts_with('(');
    let mut paragraph = Paragraph::new(content).block(theme::panel_block(title, focused));
    if is_empty {
        paragraph = paragraph.style(theme::placeholder_style());
    }
    paragraph.render(area, frame);
}

fn render_individual(state: &AppState, visible_height: usize) -> String {
    if let Some(session) = state.sessions.get(state.active_session) {
        let lines = state.log_store.lines(&session.id);
        if lines.is_empty() {
            return "(ログ出力なし)".to_string();
        }
        let start = lines.len().saturating_sub(visible_height);
        lines[start..].join("\n")
    } else {
        "(セッション未選択)".to_string()
    }
}

fn render_unified(state: &AppState, visible_height: usize) -> String {
    let mut all_lines: Vec<String> = Vec::new();

    for session in &state.sessions {
        let lines = state.log_store.lines(&session.id);
        let prefix = &session.name;
        for line in lines {
            if !line.is_empty() {
                all_lines.push(format!("[{}] {}", prefix, line));
            }
        }
    }

    if all_lines.is_empty() {
        return "(ログ出力なし)".to_string();
    }

    let start = all_lines.len().saturating_sub(visible_height);
    all_lines[start..].join("\n")
}
