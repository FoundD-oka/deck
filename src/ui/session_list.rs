use crate::app::AppState;
use ftui_core::geometry::Rect;
use ftui_render::cell::PackedRgba;
use ftui_render::frame::Frame;
use ftui_style::Style;
use ftui_widgets::block::Block;
use ftui_widgets::list::{List, ListItem, ListState};
use ftui_widgets::StatefulWidget;

pub fn render(state: &AppState, frame: &mut Frame, area: Rect, focused: bool) {
    let items: Vec<ListItem> = state
        .sessions
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let br_suffix = state
                .br_tasks
                .get(&s.id)
                .map(|info| format!(" [{}/{}]", info.done, info.total))
                .unwrap_or_default();
            let label = format!("[{}] {} {}{}", i + 1, s.status.icon(), s.name, br_suffix);
            ListItem::new(label)
        })
        .collect();

    let border_style = if focused {
        Style::new().fg(PackedRgba::rgb(0, 205, 205))
    } else {
        Style::default()
    };

    let list = List::new(items)
        .block(
            Block::bordered()
                .title("セッション")
                .border_style(border_style),
        )
        .highlight_style(Style::new().fg(PackedRgba::rgb(0, 0, 0)).bg(PackedRgba::rgb(0, 205, 205)));

    let mut list_state = ListState::default();
    list_state.select(Some(state.active_session));
    list.render(area, frame, &mut list_state);
}
