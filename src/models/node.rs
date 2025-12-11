use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Node {
    pub path: PathBuf,
    pub size: u64,
    pub children: Vec<Node>,
}

impl Node {
    pub fn new(path: PathBuf, size: u64, children: Vec<Node>) -> Self {
        Self { path, size, children }
    }

    pub fn human_size(size: u64) -> String {
        const KB: f64 = 1024.0;
        const MB: f64 = KB * 1024.0;
        const GB: f64 = MB * 1024.0;

        let s = size as f64;

        if s >= GB { format!("{:.2} GB", s / GB) }
        else if s >= MB { format!("{:.2} MB", s / MB) }
        else if s >= KB { format!("{:.2} KB", s / KB) }
        else { format!("{} B", size) }
    }

    pub fn print_tree(
        &self,
        prefix: &str,
        is_last: bool,
        max_depth: usize,
        level: usize,
        color: bool,
    ) {
        let size_str = Self::human_size(self.size);

        if level == 0 {
            println!("{}  [{}]", self.path.to_string_lossy(), size_str);
        } else {
            let branch = if is_last { "└── " } else { "├── " };
            print!("{}", prefix);
            Self::print_entry(self, branch, color);
        }

        if level >= max_depth {
            return;
        }

        let mut sorted = self.children.clone();
        sorted.sort_by(|a, b| b.size.cmp(&a.size));

        for (i, child) in sorted.iter().enumerate() {
            let last = i == sorted.len() - 1;

            let new_prefix = if level == 0 {
                String::new()
            } else if is_last {
                format!("{}    ", prefix)
            } else {
                format!("{}│   ", prefix)
            };

            child.print_tree(&new_prefix, last, max_depth, level + 1, color);
        }
    }

    fn print_entry(node: &Node, branch: &str, color: bool) {
        let name = node
            .path
            .file_name()
            .unwrap_or_else(|| node.path.as_os_str())
            .to_string_lossy();

        let is_dir = !node.children.is_empty() || node.path.is_dir();
        let size = Self::human_size(node.size);

        if color {
            if is_dir {
                // зелёный + reset
                println!("{}{}{}{}  [{}]", branch, "\x1b[32m", name, "\x1b[0m", size);
            } else {
                // белый + reset
                println!("{}{}{}{}  [{}]", branch, "\x1b[37m", name, "\x1b[0m", size);
            }
        } else {
            println!("{}{}  [{}]", branch, name, size);
        }
    }
}

