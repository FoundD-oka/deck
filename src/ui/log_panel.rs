use crate::app::{AppState, LogMode};
use ftui_core::geometry::Rect;
use ftui_render::cell::PackedRgba;
use ftui_render::frame::Frame;
use ftui_style::Style;
use ftui_widgets::block::Block;
use ftui_widgets::paragraph::Paragraph;
use ftui_widgets::Widget;

pub fn render(state: &AppState, frame: &mut Frame, area: Rect, focused: bool) {
    let border_style = if focused {
        Style::new().fg(PackedRgba::rgb(0, 205, 205))
    } else {
        Style::default()
    };

    let title = match state.log_mode {
        LogMode::Individual => "Log [Individual] (t: toggle)",
        LogMode::Unified => "Log [Unified] (t: toggle)",
    };

    let visible_height = area.height.saturating_sub(2) as usize;

    let content = match state.log_mode {
        LogMode::Individual => render_individual(state, visible_height),
        LogMode::Unified => render_unified(state, visible_height),
    };

    let paragraph = Paragraph::new(content).block(
        Block::bordered()
            .title(title)
            .border_style(border_style),
    );
    paragraph.render(area, frame);
}

fn render_individual(state: &AppState, visible_height: usize) -> String {
    if let Some(session) = state.sessions.get(state.active_session) {
        let lines = state.log_store.lines(&session.id);
        if lines.is_empty() {
            return "(no log output)".to_string();
        }
        let start = lines.len().saturating_sub(visible_height);
        lines[start..].join("\n")
    } else {
        "(no session selected)".to_string()
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
        return "(no log output)".to_string();
    }

    let start = all_lines.len().saturating_sub(visible_height);
    all_lines[start..].join("\n")
}
