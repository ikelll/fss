use std::env;

pub struct Args {
    pub path: String,
    pub depth: usize,     
    pub recursive: bool,  
}

impl Args {
    pub fn parse() -> Self {
        let mut args = env::args().skip(1);

        let mut path = ".".into();
        let mut depth: usize = 1;     
        let mut recursive = false;

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--help" | "-h" => {
                    Self::print_help();
                    std::process::exit(0);
                }

                "--depth" | "-d" => {
                    if let Some(d) = args.next() {
                        depth = d.parse::<usize>().unwrap_or(0);
                    }
                }

                "--recursive" | "-r" => {
                    recursive = true;
                }

                _ => {
                    path = arg;
                }
            }
        }

        Args { path, depth, recursive }
    }

    pub fn print_help() {
        println!(
r#"FSS — File Space Scanner

Usage:
  fss [PATH] [OPTIONS]

Options:
  -d N, --depth N     Set tree display depth. 0 = show only root
  -h, --help          Show this help message

Behavior:
  - Scanning is ALWAYS full depth (infinite).
  - Display depth is controlled by --depth.
  - Without -r only one level is shown (even if depth > 1).

Examples:
  fss /etc
  fss /var/log -d 0
"#
        );
    }

    pub fn use_color(&self) -> bool {
        if let Ok(term) = std::env::var("TERM") {
            if term.contains("xterm") || term.contains("color") || term.contains("vt") {
                return atty::is(atty::Stream::Stdout);
            }
        }
        false
    }
}

mod atty {
    use libc::{isatty, STDOUT_FILENO};

    pub enum Stream { Stdout }

    pub fn is(_stream: Stream) -> bool {
        unsafe { isatty(STDOUT_FILENO) == 1 }
    }
}

