use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub sessions_file_path: PathBuf,
    pub logs_root_path: PathBuf,
    pub needs_input_timeout_sec: u64,
    pub br_poll_interval_sec: u64,
    pub editor: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("deck");
        Self {
            sessions_file_path: config_dir.join("sessions.json"),
            logs_root_path: config_dir.join("logs"),
            needs_input_timeout_sec: 30,
            br_poll_interval_sec: 3,
            editor: std::env::var("EDITOR").unwrap_or_else(|_| "vim".to_string()),
        }
    }
}

impl AppConfig {
    pub fn ensure_dirs(&self) -> std::io::Result<()> {
        if let Some(parent) = self.sessions_file_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::create_dir_all(&self.logs_root_path)?;
        Ok(())
    }
}
