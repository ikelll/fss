use std::env;

pub struct Args {
    pub path: String,
    pub depth: usize,
}

impl Args {
    pub fn parse() -> Self {
        let mut args = env::args().skip(1);

        let mut path = ".".to_string();
        let mut depth = usize::MAX;

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--depth" => {
                    if let Some(d) = args.next() {
                        depth = d.parse().unwrap_or(usize::MAX);
                    }
                }
                _ => {
                    path = arg;
                }
            }
        }

        Args { path, depth }
    }

    pub fn use_color(&self) -> bool {
        if let Ok(term) = std::env::var("TERM") {
            if term.contains("xterm") || term.contains("vt100") || term.contains("color") {
                return atty::is(atty::Stream::Stdout);
            }
        }
        false
    }
}


mod atty {
    use libc::{isatty, STDOUT_FILENO};

    pub enum Stream {
        Stdout,
    }

    pub fn is(stream: Stream) -> bool {
        match stream {
            Stream::Stdout => unsafe { isatty(STDOUT_FILENO) == 1 },
        }
    }
}
