mod cli;
mod models;
mod scanner;

use cli::args::Args;
use scanner::scan_dir;

use std::path::Path;

fn main() {
    let args = Args::parse();

    let path = Path::new(&args.path);

    let root = scan_dir(path, args.depth);

    root.print_tree(0, args.use_color());
}
