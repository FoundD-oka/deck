mod app;
mod br_poller;
mod config;
mod dir_tree;
mod file_preview;
mod input_history;
mod log_store;
mod needs_input;
mod persistence;
mod pty_manager;
mod session;
mod ui;

use app::AppState;
use config::AppConfig;
use ftui_runtime::{App, ScreenMode};

fn main() -> std::io::Result<()> {
    // Panic hook: restore terminal on panic
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        use std::io::Write;
        // Leave alternate screen + show cursor via raw ANSI escape sequences
        let _ = std::io::stdout().write_all(b"\x1b[?1049l\x1b[?25h");
        let _ = std::io::stdout().flush();
        default_hook(info);
    }));

    let config = AppConfig::default();
    config.ensure_dirs()?;

    let model = AppState::new(config);

    App::new(model)
        .screen_mode(ScreenMode::AltScreen)
        .run()
}
