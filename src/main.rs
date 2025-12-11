use std::path::PathBuf;
use clap::Parser;

mod models;
use models::node::Node;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(default_value = ".")]
    path: String,

    #[arg(short = 'd', long, default_value = "0")]
    depth: usize,
}


fn build_tree(path: &PathBuf) -> Node {
    let metadata = std::fs::metadata(path).unwrap();
    let mut size = metadata.len();
    let mut children = Vec::new();

    if metadata.is_dir() {
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                let child_path = entry.path();
                let child = build_tree(&child_path);
                size += child.size;
                children.push(child);
            }
        }
    }

    Node::new(path.clone(), size, children)
}

fn main() {
    let args = Args::parse();

    let root_path = PathBuf::from(&args.path);

    if !root_path.exists() {
        eprintln!("oшибка: путь '{}' не существует", args.path);
        std::process::exit(1);
    }

    let root = build_tree(&root_path);

    root.print_tree(
        "",              // prefix
        true,            // is_last
        args.depth,      // max_depth
        0,               // level
        true // color
    );
}

