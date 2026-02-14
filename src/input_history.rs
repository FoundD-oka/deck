pub struct InputHistory {
    entries: Vec<String>,
    cursor: usize,
}

impl InputHistory {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            cursor: 0,
        }
    }

    pub fn push(&mut self, entry: String) {
        if !entry.is_empty() {
            self.entries.push(entry);
        }
        self.cursor = self.entries.len();
    }

    /// Move cursor up (older entries). Returns the entry text if available.
    pub fn up(&mut self) -> Option<&str> {
        if self.cursor > 0 {
            self.cursor -= 1;
            self.entries.get(self.cursor).map(|s| s.as_str())
        } else {
            self.entries.first().map(|s| s.as_str())
        }
    }

    /// Move cursor down (newer entries). Returns the entry text, or None to clear input.
    pub fn down(&mut self) -> Option<&str> {
        if self.cursor + 1 < self.entries.len() {
            self.cursor += 1;
            self.entries.get(self.cursor).map(|s| s.as_str())
        } else {
            self.cursor = self.entries.len();
            None
        }
    }

    #[cfg(test)]
    fn reset_cursor(&mut self) {
        self.cursor = self.entries.len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_history() {
        let mut h = InputHistory::new();
        assert!(h.up().is_none());
        assert!(h.down().is_none());
    }

    #[test]
    fn push_and_recall() {
        let mut h = InputHistory::new();
        h.push("first".to_string());
        h.push("second".to_string());

        assert_eq!(h.up(), Some("second"));
        assert_eq!(h.up(), Some("first"));
        // Stay at oldest
        assert_eq!(h.up(), Some("first"));

        assert_eq!(h.down(), Some("second"));
        // Past newest returns None (new input mode)
        assert_eq!(h.down(), None);
    }

    #[test]
    fn reset_cursor() {
        let mut h = InputHistory::new();
        h.push("a".to_string());
        h.push("b".to_string());

        h.up();
        h.up();
        h.reset_cursor();

        // After reset, up returns the last entry
        assert_eq!(h.up(), Some("b"));
    }

    #[test]
    fn ignores_empty_entries() {
        let mut h = InputHistory::new();
        h.push("".to_string());
        assert!(h.up().is_none());
    }
}
