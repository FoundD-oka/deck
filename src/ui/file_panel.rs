use crate::app::AppState;
use crate::ui::theme;
use ftui_core::geometry::Rect;
use ftui_render::frame::Frame;
use ftui_widgets::paragraph::Paragraph;
use ftui_widgets::Widget;

pub fn render(state: &AppState, frame: &mut Frame, area: Rect, focused: bool) {
    let title = if state.file_preview.path.is_some() {
        "File (e:edit)"
    } else {
        "File"
    };

    let visible_height = area.height.saturating_sub(2) as usize;
    let content = state.file_preview.visible_content(visible_height);

    let mut paragraph = Paragraph::new(content).block(theme::panel_block(title, focused));
    if state.file_preview.path.is_none() {
        paragraph = paragraph.style(theme::placeholder_style());
    }
    paragraph.render(area, frame);
}
