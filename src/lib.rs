use std::io::stderr;
use std::io::Write;
use std::path::PathBuf;

pub fn walk_dirs(settings: &settings::Settings, result_dirs: &mut Vec<PathBuf>) {
    let mut depth = settings.depth();
    let capacity: usize = if depth >= 20 {
        2usize.pow(20)
    } else {
        2usize.pow((depth + 1) as u32)
    };

    let mut working_dirs: Vec<PathBuf> = Vec::with_capacity(capacity);

    working_dirs.push(settings.base());

    let mut err_stream = std::io::BufWriter::new(stderr());

    let mut head = 0;
    let mut tail = working_dirs.len() - 1;

    while depth > 0 {
        let prev_len = working_dirs.len();

        for i in head..=tail {
            let path: &PathBuf = working_dirs.get(i).unwrap();
            let read_dir_res = std::fs::read_dir(path);

            if read_dir_res.is_err() {
                write!(
                    err_stream,
                    "could not open {}: {}\n",
                    path.display(),
                    read_dir_res.err().unwrap().to_string()
                )
                .unwrap();
                continue;
            }

            for entry_res in read_dir_res.unwrap() {
                let entry = entry_res.unwrap();
                let path = entry.path();

                // is_dir will traverse sym link, tested this!
                // check for symlink is necessary even though docs say otherwise
                if path.is_dir() && !path.is_symlink() {
                    if depth == 1 {
                        result_dirs.push(entry.path());
                    } else {
                        working_dirs.push(path);
                    }
                }
            }
        }

        if working_dirs.len() == prev_len {
            // no more directories to recurse
            break;
        }

        head = tail + 1;
        tail = working_dirs.len() - 1;

        depth -= 1;
    }

    err_stream.flush().unwrap();
}

pub mod settings {
    use std::io::Write;
    use std::{fs, io::stderr, path::PathBuf, process::exit};

    pub struct Settings {
        depth: u32,
        base: PathBuf,
    }

    impl Settings {
        pub fn from_base(base: String) -> Settings {
            let base = fs::canonicalize(base);

            if base.is_err() {
                write!(
                    stderr(),
                    "base could not be resolved: {}",
                    base.err().unwrap()
                )
                .unwrap();
                exit(1);
            }

            return Settings {
                base: base.unwrap(),
                depth: 1,
            };
        }

        pub fn from_args(args: std::env::Args) -> Settings {
            let mut args_iter = args.skip(1);
            let base_dir = args_iter.next();

            if base_dir.is_none() {
                write!(stderr(), "no base directory given\n").unwrap();
                exit(1);
            }

            let mut settings: Settings = Settings::from_base(base_dir.unwrap());

            let depth: u32 = match args_iter.next() {
                Some(d) => {
                    let parsed = d.parse::<u32>();
                    parsed.unwrap_or(1)
                }
                None => 1,
            };

            settings.depth = depth;

            return settings;
        }

        pub fn base(&self) -> PathBuf {
            return self.base.clone();
        }

        pub fn depth(&self) -> usize {
            return self.depth as usize;
        }

        pub fn set_depth(&mut self, d: usize) {
            self.depth = d as u32;
        }
    }

    impl std::fmt::Display for Settings {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            return write!(f, "check {} with depth {}", self.base.display(), self.depth);
        }
    }
}
