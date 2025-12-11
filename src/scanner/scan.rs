use std::fs;
use std::path::{Path, PathBuf};

use crate::models::Node;

pub fn scan_dir(path: &Path, max_depth: usize) -> Node {
    scan_recursive(path, 0, max_depth)
}

fn scan_recursive(path: &Path, depth: usize, max_depth: usize) -> Node {
    let mut total_size = 0;
    let mut children = Vec::new();

    if depth < max_depth {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let path: PathBuf = entry.path();

                if path.is_dir() {
                    let child = scan_recursive(&path, depth + 1, max_depth);
                    total_size += child.size;
                    children.push(child);
                } else if let Ok(metadata) = fs::metadata(&path) {
                    let size = metadata.len();
                    total_size += size;

                    children.push(Node::new(path.clone(), size, vec![]));
                }
            }
        }
    }

    Node::new(path.to_path_buf(), total_size, children)
}
