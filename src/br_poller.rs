use std::path::Path;
use std::process::Command;

pub struct BrTaskInfo {
    pub done: usize,
    pub total: usize,
}

/// Poll br task counts for a session's root directory.
/// Returns None if .beads/ doesn't exist or br is unavailable.
pub fn poll(root_path: &Path) -> Option<BrTaskInfo> {
    if !root_path.join(".beads").is_dir() {
        return None;
    }

    let output = Command::new("br")
        .args(["list", "--json", "--all"])
        .current_dir(root_path)
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    parse_output(&output.stdout)
}

fn parse_output(data: &[u8]) -> Option<BrTaskInfo> {
    let tasks: Vec<serde_json::Value> = serde_json::from_slice(data).ok()?;
    let total = tasks.len();
    let done = tasks
        .iter()
        .filter(|t| t.get("status").and_then(|s| s.as_str()) == Some("closed"))
        .count();
    Some(BrTaskInfo { done, total })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty_array() {
        let info = parse_output(b"[]").unwrap();
        assert_eq!(info.done, 0);
        assert_eq!(info.total, 0);
    }

    #[test]
    fn parse_mixed_tasks() {
        let json = r#"[
            {"id":"bd-1","status":"open"},
            {"id":"bd-2","status":"closed"},
            {"id":"bd-3","status":"open"},
            {"id":"bd-4","status":"closed"}
        ]"#;
        let info = parse_output(json.as_bytes()).unwrap();
        assert_eq!(info.done, 2);
        assert_eq!(info.total, 4);
    }

    #[test]
    fn parse_invalid_json() {
        assert!(parse_output(b"not json").is_none());
    }

    #[test]
    fn no_beads_dir() {
        let result = poll(Path::new("/nonexistent/path"));
        assert!(result.is_none());
    }
}
