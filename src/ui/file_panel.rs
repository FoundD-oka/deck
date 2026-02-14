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

    let title = if state.file_preview.path.is_some() {
        "ファイル (e:編集)"
    } else {
        "ファイル"
    };

    let visible_height = area.height.saturating_sub(2) as usize;
    let content = state.file_preview.visible_content(visible_height);

    let paragraph = Paragraph::new(content).block(
        Block::bordered()
            .title(title)
            .border_style(border_style),
    );
    paragraph.render(area, frame);
}
