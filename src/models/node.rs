use std::path::PathBuf;

#[derive(Debug)]
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

    pub fn print_tree(&self, indent: usize, color: bool) {
        let size_str = Self::human_size(self.size);

        let indent_str = " ".repeat(indent * 2);
        let name = self.path.file_name()
            .unwrap_or_else(|| self.path.as_os_str())
            .to_string_lossy();

        if color {
            println!("{}{}{}{}",
                indent_str,
                "\x1b[32m", 
                name,
                "\x1b[0m"
            );
        } else {
            println!("{}{}", indent_str, name);
        }

        println!("{}  [{}]", indent_str, size_str);

        let mut sorted = self.children.clone();
        sorted.sort_by(|a, b| b.size.cmp(&a.size)); 

        for child in sorted {
            child.print_tree(indent + 1, color);
        }
    }
}
