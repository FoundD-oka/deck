use crate::session::{Session, SessionStatus};
use std::path::Path;

pub fn load_sessions(path: &Path) -> Vec<Session> {
    let Ok(data) = std::fs::read_to_string(path) else {
        return Vec::new();
    };
    let mut sessions: Vec<Session> = serde_json::from_str(&data).unwrap_or_default();
    // Reset running/needs_input sessions to queued (PTY processes are gone after restart)
    for s in &mut sessions {
        if s.status == SessionStatus::Running || s.status == SessionStatus::NeedsInput {
            s.status = SessionStatus::Queued;
            s.pty_pid = None;
        }
    }
    sessions
}

pub fn save_sessions(path: &Path, sessions: &[Session]) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(sessions)?;
    std::fs::write(path, json)
}

pub fn write_log_header(log_path: &Path, session: &Session) -> std::io::Result<()> {
    use std::io::Write;
    if let Some(parent) = log_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut file = std::fs::File::create(log_path)?;
    writeln!(file, "# Session: {}", session.name)?;
    writeln!(file, "# Directory: {}", session.root_path.display())?;
    writeln!(file, "# Started: {}", session.created_at)?;
    writeln!(file, "---")?;
    Ok(())
}

pub fn append_log(log_path: &Path, data: &[u8]) -> std::io::Result<()> {
    use std::io::Write;
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)?;
    file.write_all(data)
}
