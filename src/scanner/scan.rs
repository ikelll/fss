use std::fs;
use std::path::{Path, PathBuf};

use crate::models::Node;

pub fn scan_dir(path: &Path) -> Node {
    scan_recursive(path)
}

fn scan_recursive(path: &Path) -> Node {
    let mut total_size = 0u64;
    let mut children = Vec::new();

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let p: PathBuf = entry.path();

            if p.is_dir() {
                let child = scan_recursive(&p);
                total_size += child.size;
                children.push(child);
            } else if let Ok(metadata) = fs::metadata(&p) {
                let size = metadata.len();
                total_size += size;
                children.push(Node::new(p, size, vec![]));
            }
        }
    }

    Node::new(path.to_path_buf(), total_size, children)
}

