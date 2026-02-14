use std::fs;
use std::path::{Path, PathBuf};

const MAX_FILE_SIZE: u64 = 1_048_576; // 1MB

pub struct FilePreview {
    pub path: Option<PathBuf>,
    pub content: Option<String>,
    pub scroll: usize,
    pub total_lines: usize,
}

impl FilePreview {
    pub fn new() -> Self {
        Self {
            path: None,
            content: None,
            scroll: 0,
            total_lines: 0,
        }
    }

    pub fn load(&mut self, path: &Path) {
        self.path = Some(path.to_path_buf());
        self.scroll = 0;

        match fs::metadata(path) {
            Ok(meta) => {
                if meta.len() > MAX_FILE_SIZE {
                    self.content = Some(format!(
                        "(ファイルが大きすぎます: {} バイト、上限 {})",
                        meta.len(),
                        MAX_FILE_SIZE
                    ));
                    self.total_lines = 1;
                    return;
                }
            }
            Err(e) => {
                self.content = Some(format!("(読み取り不可: {})", e));
                self.total_lines = 1;
                return;
            }
        }

        match fs::read(path) {
            Ok(bytes) => {
                let is_binary = bytes.iter().take(8192).any(|&b| b == 0);
                if is_binary {
                    self.content = Some("(バイナリファイル)".to_string());
                    self.total_lines = 1;
                } else {
                    let text = String::from_utf8_lossy(&bytes).to_string();
                    self.total_lines = text.lines().count().max(1);
                    self.content = Some(text);
                }
            }
            Err(e) => {
                self.content = Some(format!("(読み取り不可: {})", e));
                self.total_lines = 1;
            }
        }
    }

    pub fn clear(&mut self) {
        self.path = None;
        self.content = None;
        self.scroll = 0;
        self.total_lines = 0;
    }

    pub fn scroll_up(&mut self) {
        if self.scroll > 0 {
            self.scroll -= 1;
        }
    }

    pub fn scroll_down(&mut self, visible_height: usize) {
        if self.total_lines > visible_height && self.scroll + visible_height < self.total_lines {
            self.scroll += 1;
        }
    }

    pub fn visible_content(&self, visible_height: usize) -> String {
        match &self.content {
            Some(text) => {
                let lines: Vec<&str> = text.lines().collect();
                let start = self.scroll.min(lines.len());
                let end = (start + visible_height).min(lines.len());
                lines[start..end].join("\n")
            }
            None => "(ファイル未選択)".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn load_text_file() {
        let mut f = NamedTempFile::new().unwrap();
        write!(f, "line1\nline2\nline3").unwrap();

        let mut preview = FilePreview::new();
        preview.load(f.path());
        assert!(preview.content.is_some());
        assert_eq!(preview.total_lines, 3);
    }

    #[test]
    fn scroll_navigation() {
        let mut f = NamedTempFile::new().unwrap();
        for i in 0..100 {
            writeln!(f, "line {}", i).unwrap();
        }

        let mut preview = FilePreview::new();
        preview.load(f.path());

        assert_eq!(preview.scroll, 0);
        preview.scroll_down(10);
        assert_eq!(preview.scroll, 1);
        preview.scroll_up();
        assert_eq!(preview.scroll, 0);
        preview.scroll_up();
        assert_eq!(preview.scroll, 0);
    }

    #[test]
    fn clear_preview() {
        let mut f = NamedTempFile::new().unwrap();
        write!(f, "hello").unwrap();

        let mut preview = FilePreview::new();
        preview.load(f.path());
        assert!(preview.content.is_some());

        preview.clear();
        assert!(preview.content.is_none());
        assert!(preview.path.is_none());
    }

    #[test]
    fn visible_content_with_scroll() {
        let mut f = NamedTempFile::new().unwrap();
        writeln!(f, "a").unwrap();
        writeln!(f, "b").unwrap();
        writeln!(f, "c").unwrap();
        writeln!(f, "d").unwrap();
        writeln!(f, "e").unwrap();

        let mut preview = FilePreview::new();
        preview.load(f.path());

        let visible = preview.visible_content(3);
        assert!(visible.starts_with('a'));

        preview.scroll_down(3);
        let visible = preview.visible_content(3);
        assert!(visible.starts_with('b'));
    }

    #[test]
    fn binary_file_detection() {
        let mut f = NamedTempFile::new().unwrap();
        f.write_all(&[0x00, 0x01, 0x02, 0xFF]).unwrap();

        let mut preview = FilePreview::new();
        preview.load(f.path());
        assert_eq!(preview.content.as_deref(), Some("(バイナリファイル)"));
    }
}
