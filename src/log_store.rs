use std::collections::HashMap;
use uuid::Uuid;

const MAX_LINES: usize = 10_000;

pub struct LogStore {
    logs: HashMap<Uuid, Vec<String>>,
}

impl LogStore {
    pub fn new() -> Self {
        Self {
            logs: HashMap::new(),
        }
    }

    /// Append raw PTY output bytes to the session's log buffer.
    /// Handles partial lines by appending to the last line if it was incomplete.
    pub fn append(&mut self, session_id: Uuid, data: &[u8]) {
        let text = String::from_utf8_lossy(data);
        let buffer = self.logs.entry(session_id).or_default();
        let parts: Vec<&str> = text.split('\n').collect();

        if parts.is_empty() {
            return;
        }

        // First part continues the last (possibly partial) line
        if let Some(last) = buffer.last_mut() {
            last.push_str(parts[0].trim_end_matches('\r'));
        } else {
            buffer.push(parts[0].trim_end_matches('\r').to_string());
        }

        // Remaining parts are new lines
        for part in &parts[1..] {
            buffer.push(part.trim_end_matches('\r').to_string());
        }

        // Trim oldest lines if over limit
        if buffer.len() > MAX_LINES {
            let drain_count = buffer.len() - MAX_LINES;
            buffer.drain(..drain_count);
        }
    }

    pub fn lines(&self, session_id: &Uuid) -> &[String] {
        self.logs
            .get(session_id)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Get the last non-empty line for needs_input pattern detection.
    pub fn last_non_empty_line(&self, session_id: &Uuid) -> Option<&str> {
        self.logs
            .get(session_id)
            .and_then(|lines| lines.iter().rev().find(|l| !l.is_empty()))
            .map(|s| s.as_str())
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append_single_line() {
        let mut store = LogStore::new();
        let id = Uuid::new_v4();
        store.append(id, b"hello world\n");
        let lines = store.lines(&id);
        assert_eq!(lines, &["hello world", ""]);
    }

    #[test]
    fn append_partial_lines() {
        let mut store = LogStore::new();
        let id = Uuid::new_v4();
        store.append(id, b"hello ");
        store.append(id, b"world\n");
        let lines = store.lines(&id);
        assert_eq!(lines, &["hello world", ""]);
    }

    #[test]
    fn append_multiple_lines() {
        let mut store = LogStore::new();
        let id = Uuid::new_v4();
        store.append(id, b"line1\nline2\nline3\n");
        let lines = store.lines(&id);
        assert_eq!(lines, &["line1", "line2", "line3", ""]);
    }

    #[test]
    fn empty_session() {
        let store = LogStore::new();
        let id = Uuid::new_v4();
        assert!(store.lines(&id).is_empty());
    }

    #[test]
    fn handles_crlf() {
        let mut store = LogStore::new();
        let id = Uuid::new_v4();
        store.append(id, b"hello\r\nworld\r\n");
        let lines = store.lines(&id);
        assert_eq!(lines, &["hello", "world", ""]);
    }
}
