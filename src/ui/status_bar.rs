use crate::app::AppState;
use crate::session::SessionStatus;
use ftui_core::geometry::Rect;
use ftui_render::cell::PackedRgba;
use ftui_render::frame::Frame;
use ftui_style::Style;
use ftui_widgets::paragraph::Paragraph;
use ftui_widgets::Widget;

pub fn render(state: &AppState, frame: &mut Frame, area: Rect) {
    let mut running = 0u32;
    let mut queued = 0u32;
    let mut done = 0u32;
    let mut failed = 0u32;
    let mut needs_input = 0u32;

    for s in &state.sessions {
        match s.status {
            SessionStatus::Running => running += 1,
            SessionStatus::Queued => queued += 1,
            SessionStatus::Done => done += 1,
            SessionStatus::Failed => failed += 1,
            SessionStatus::NeedsInput => needs_input += 1,
        }
    }

    let text = format!(
        " Running: {} | Queued: {} | Done: {} | Error: {} | Input: {} | q:quit n:new d:del r:rename m:input-toggle",
        running, queued, done, failed, needs_input
    );

    let paragraph = Paragraph::new(text)
        .style(Style::new().fg(PackedRgba::rgb(229, 229, 229)).bg(PackedRgba::rgb(80, 80, 80)));
    paragraph.render(area, frame);
}
