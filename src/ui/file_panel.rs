use crate::app::AppState;
use ftui_core::geometry::Rect;
use ftui_render::cell::PackedRgba;
use ftui_render::frame::Frame;
use ftui_style::Style;
use ftui_widgets::block::Block;
use ftui_widgets::paragraph::Paragraph;
use ftui_widgets::Widget;

pub fn render(_state: &AppState, frame: &mut Frame, area: Rect, focused: bool) {
    let border_style = if focused {
        Style::new().fg(PackedRgba::rgb(0, 205, 205))
    } else {
        Style::default()
    };

    let paragraph = Paragraph::new("Select a file to preview").block(
        Block::bordered()
            .title("File Preview")
            .border_style(border_style),
    );
    paragraph.render(area, frame);
}
