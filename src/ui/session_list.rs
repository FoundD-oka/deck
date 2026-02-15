use crate::app::AppState;
use crate::ui::theme;
use ftui_core::geometry::Rect;
use ftui_render::frame::Frame;
use ftui_style::Style;
use ftui_widgets::borders::Borders;
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

    // 下ボーダーを外す（Directory パネルの上ボーダーと接合してズレるのを防ぐ）
    let borders = Borders::TOP | Borders::LEFT | Borders::RIGHT;
    let list = List::new(items)
        .block(theme::panel_block_with("Sessions", focused, borders))
        .highlight_style(
            Style::new()
                .fg(theme::HIGHLIGHT_FG)
                .bg(theme::HIGHLIGHT_BG),
        );

    let mut list_state = ListState::default();
    list_state.select(Some(state.active_session));
    list.render(area, frame, &mut list_state);
}
