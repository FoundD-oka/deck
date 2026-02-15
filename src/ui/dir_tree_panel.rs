use crate::app::AppState;
use crate::ui::theme;
use ftui_core::geometry::Rect;
use ftui_render::frame::Frame;
use ftui_widgets::paragraph::Paragraph;
use ftui_widgets::Widget;

pub fn render(state: &AppState, frame: &mut Frame, area: Rect, focused: bool) {
    let title = if state.dir_tree.show_hidden {
        "Directory [hidden] (h:toggle)"
    } else {
        "Directory (h:hidden)"
    };

    let visible_height = area.height.saturating_sub(2) as usize;
    let flat = state.dir_tree.flatten();

    if flat.is_empty() {
        let paragraph = Paragraph::new("(セッション未選択)")
            .style(theme::placeholder_style())
            .block(theme::junction_panel_block(title, focused));
        paragraph.render(area, frame);
        return;
    }

    let cursor = state.dir_tree.cursor;

    let scroll_offset = if cursor >= visible_height {
        cursor - visible_height + 1
    } else {
        0
    };

    let mut lines: Vec<String> = Vec::new();
    for (i, entry) in flat.iter().enumerate().skip(scroll_offset).take(visible_height) {
        let indent = "  ".repeat(entry.depth);
        let icon = if entry.is_dir {
            if entry.expanded {
                "- "
            } else {
                "+ "
            }
        } else {
            "  "
        };
        let marker = if i == cursor { ">" } else { " " };
        lines.push(format!("{}{}{}{}", marker, indent, icon, entry.name));
    }

    let content = lines.join("\n");
    let paragraph = Paragraph::new(content).block(theme::junction_panel_block(title, focused));
    paragraph.render(area, frame);
}
