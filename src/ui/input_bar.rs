use crate::app::AppState;
use crate::ui::theme;
use ftui_core::geometry::Rect;
use ftui_render::frame::Frame;
use ftui_widgets::paragraph::Paragraph;
use ftui_widgets::Widget;

pub fn render(state: &AppState, frame: &mut Frame, area: Rect, focused: bool) {
    let prompt = if let Some(session) = state.sessions.get(state.active_session) {
        format!("[{}] > {}", session.name, state.input_text)
    } else {
        format!("> {}", state.input_text)
    };

    let paragraph = Paragraph::new(prompt).block(theme::panel_block("Input", focused));
    paragraph.render(area, frame);
}
