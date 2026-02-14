use std::fs;
use std::path::{Path, PathBuf};

const MAX_DEPTH: usize = 5;

struct DirTreeNode {
    name: String,
    path: PathBuf,
    is_dir: bool,
    depth: usize,
    expanded: bool,
    children: Vec<DirTreeNode>,
}

pub struct FlatEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub depth: usize,
    pub expanded: bool,
}

pub struct DirTree {
    root: Option<DirTreeNode>,
    pub cursor: usize,
    pub show_hidden: bool,
    flat_cache: Vec<FlatEntry>,
}

impl DirTree {
    pub fn new(root_path: &Path) -> Self {
        let mut tree = Self {
            root: None,
            cursor: 0,
            show_hidden: false,
            flat_cache: Vec::new(),
        };
        tree.set_root(root_path);
        tree
    }

    pub fn empty() -> Self {
        Self {
            root: None,
            cursor: 0,
            show_hidden: false,
            flat_cache: Vec::new(),
        }
    }

    pub fn set_root(&mut self, root_path: &Path) {
        if root_path.is_dir() {
            let name = root_path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| root_path.display().to_string());
            let mut node = DirTreeNode {
                name,
                path: root_path.to_path_buf(),
                is_dir: true,
                depth: 0,
                expanded: true,
                children: Vec::new(),
            };
            load_children(&mut node, self.show_hidden);
            self.root = Some(node);
            self.cursor = 0;
            self.rebuild_flat();
        }
    }

    pub fn flatten(&self) -> &[FlatEntry] {
        &self.flat_cache
    }

    fn rebuild_flat(&mut self) {
        self.flat_cache.clear();
        if let Some(root) = &self.root {
            flatten_node(root, &mut self.flat_cache, self.show_hidden);
        }
    }

    pub fn cursor_up(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    pub fn cursor_down(&mut self) {
        if self.cursor + 1 < self.flat_cache.len() {
            self.cursor += 1;
        }
    }

    pub fn selected_path(&self) -> Option<&Path> {
        self.flat_cache.get(self.cursor).map(|e| e.path.as_path())
    }

    pub fn selected_is_dir(&self) -> bool {
        self.flat_cache
            .get(self.cursor)
            .map(|e| e.is_dir)
            .unwrap_or(false)
    }

    pub fn toggle(&mut self) {
        if let Some(entry) = self.flat_cache.get(self.cursor) {
            if entry.is_dir {
                let path = entry.path.clone();
                let show_hidden = self.show_hidden;
                if let Some(root) = &mut self.root {
                    toggle_node(root, &path, show_hidden);
                }
                self.rebuild_flat();
            }
        }
    }

    pub fn toggle_hidden(&mut self) {
        self.show_hidden = !self.show_hidden;
        if let Some(root) = &mut self.root {
            reload_tree(root, self.show_hidden);
        }
        self.rebuild_flat();
        if self.cursor >= self.flat_cache.len() {
            self.cursor = self.flat_cache.len().saturating_sub(1);
        }
    }
}

fn load_children(node: &mut DirTreeNode, show_hidden: bool) {
    if !node.is_dir || node.depth >= MAX_DEPTH {
        return;
    }
    node.children.clear();
    if let Ok(entries) = fs::read_dir(&node.path) {
        let mut dirs: Vec<DirTreeNode> = Vec::new();
        let mut files: Vec<DirTreeNode> = Vec::new();

        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if !show_hidden && name.starts_with('.') {
                continue;
            }
            let path = entry.path();
            let is_dir = path.is_dir();
            let child = DirTreeNode {
                name,
                path,
                is_dir,
                depth: node.depth + 1,
                expanded: false,
                children: Vec::new(),
            };
            if is_dir {
                dirs.push(child);
            } else {
                files.push(child);
            }
        }

        dirs.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        files.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

        node.children.extend(dirs);
        node.children.extend(files);
    }
}

fn flatten_node(node: &DirTreeNode, out: &mut Vec<FlatEntry>, show_hidden: bool) {
    if !show_hidden && node.name.starts_with('.') && node.depth > 0 {
        return;
    }
    out.push(FlatEntry {
        name: node.name.clone(),
        path: node.path.clone(),
        is_dir: node.is_dir,
        depth: node.depth,
        expanded: node.expanded,
    });
    if node.expanded {
        for child in &node.children {
            flatten_node(child, out, show_hidden);
        }
    }
}

fn toggle_node(node: &mut DirTreeNode, target_path: &Path, show_hidden: bool) -> bool {
    if node.path == target_path {
        node.expanded = !node.expanded;
        if node.expanded && node.children.is_empty() {
            load_children(node, show_hidden);
        }
        return true;
    }
    if node.expanded {
        for child in &mut node.children {
            if toggle_node(child, target_path, show_hidden) {
                return true;
            }
        }
    }
    false
}

fn reload_tree(node: &mut DirTreeNode, show_hidden: bool) {
    if node.is_dir && node.expanded {
        load_children(node, show_hidden);
        for child in &mut node.children {
            reload_tree(child, show_hidden);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn empty_tree() {
        let tree = DirTree::empty();
        assert!(tree.flatten().is_empty());
        assert!(tree.selected_path().is_none());
    }

    #[test]
    fn loads_directory() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir(tmp.path().join("subdir")).unwrap();
        fs::write(tmp.path().join("file.txt"), "hello").unwrap();

        let tree = DirTree::new(tmp.path());
        let flat = tree.flatten();
        assert!(flat.len() >= 3); // root + subdir + file
        assert!(flat[0].is_dir); // root
    }

    #[test]
    fn dirs_before_files() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("aaa.txt"), "").unwrap();
        fs::create_dir(tmp.path().join("zzz_dir")).unwrap();

        let tree = DirTree::new(tmp.path());
        let flat = tree.flatten();
        let children: Vec<_> = flat.iter().skip(1).collect();
        if children.len() >= 2 {
            assert!(children[0].is_dir); // zzz_dir comes first
            assert!(!children[1].is_dir); // aaa.txt
        }
    }

    #[test]
    fn hidden_files_toggle() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join(".hidden"), "").unwrap();
        fs::write(tmp.path().join("visible.txt"), "").unwrap();

        let mut tree = DirTree::new(tmp.path());
        let without_hidden = tree.flatten().len();

        tree.toggle_hidden();
        let with_hidden = tree.flatten().len();
        assert!(with_hidden > without_hidden);
    }

    #[test]
    fn collapse_and_expand() {
        let tmp = TempDir::new().unwrap();
        let sub = tmp.path().join("subdir");
        fs::create_dir(&sub).unwrap();
        fs::write(sub.join("inner.txt"), "").unwrap();

        let mut tree = DirTree::new(tmp.path());
        let initial = tree.flatten().len();

        // Root is at cursor 0, toggle to collapse
        tree.cursor = 0;
        tree.toggle();
        assert!(tree.flatten().len() < initial);

        // Toggle again to expand
        tree.toggle();
        assert_eq!(tree.flatten().len(), initial);
    }

    #[test]
    fn cursor_navigation() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("a.txt"), "").unwrap();
        fs::write(tmp.path().join("b.txt"), "").unwrap();

        let mut tree = DirTree::new(tmp.path());
        assert_eq!(tree.cursor, 0);

        tree.cursor_down();
        assert_eq!(tree.cursor, 1);

        tree.cursor_up();
        assert_eq!(tree.cursor, 0);

        // Can't go below 0
        tree.cursor_up();
        assert_eq!(tree.cursor, 0);
    }
}
