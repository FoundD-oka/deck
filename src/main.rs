mod app;
mod config;
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
    let config = AppConfig::default();
    config.ensure_dirs()?;

    let model = AppState::new(config);

    App::new(model)
        .screen_mode(ScreenMode::AltScreen)
        .run()
}
