use crate::app::AppState;
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

    let prompt = if let Some(session) = state.sessions.get(state.active_session) {
        format!("[{}] > {}", session.name, state.input_text)
    } else {
        format!("> {}", state.input_text)
    };

    let paragraph = Paragraph::new(prompt).block(
        Block::bordered()
            .title("入力")
            .border_style(border_style),
    );
    paragraph.render(area, frame);
}
