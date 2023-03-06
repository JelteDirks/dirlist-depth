use std::fs::read_dir;
use std::io::BufWriter;
use std::io::Stderr;
use std::io::Write;
use std::path::PathBuf;
use std::process::exit;

fn main() {
    let mut args_iter = std::env::args().skip(1);
    let mut err_writer = BufWriter::new(std::io::stderr());

    let base_dir = args_iter.next();

    if base_dir.is_none() {
        write!(err_writer, "no base directory given\n").unwrap();
        err_writer.flush().unwrap();
        exit(1);
    }

    let mut settings: Settings = Settings::from_base(base_dir.unwrap());

    let depth = args_iter.next();

    if depth.is_none() {
        write!(err_writer, "no depth is given, use default 1\n").unwrap();
    }

    let depth: u32 = match depth {
        Some(d) => {
            let parsed = d.parse::<u32>();
            if parsed.is_err() {
                write!(err_writer, "problem parsing depth\n").unwrap();
                exit(2);
            }
            parsed.unwrap()
        }
        None => 1,
    };

    settings.set_depth(depth);

    let mut results = Vec::with_capacity(depth.pow(2) as usize);

    results.push(settings.base);

    walk_dirs(&mut settings.depth, &mut results, &mut err_writer);

    err_writer.flush().unwrap();
}

struct Settings {
    depth: u32,
    base: PathBuf,
}

impl Settings {
    fn from_base(base: String) -> Settings {
        return Settings {
            base: PathBuf::from(base),
            depth: 1,
        }
    }

    fn set_depth(&mut self, depth: u32) {
        self.depth = depth;
    }
}

fn walk_dirs(depth: &mut u32, results: &mut Vec<PathBuf>, err_stream: &mut BufWriter<Stderr>) {
    let mut head = 0;
    let mut tail = results.len() - 1;

    let mut out_stream = std::io::stdout().lock();

    while *depth > 0 {
        let prev_len = results.len();

        for i in head..=tail {
            let path: &PathBuf = results.get(i).unwrap();
            let read_dir_res = read_dir(path);

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
                    if *depth == 1 {
                        write!(out_stream, "{}\n", &path.display()).unwrap();
                    }
                    results.push(path);
                }
            }
        }

        if results.len() == prev_len {
            // no more directories to recurse
            break;
        }

        head = tail + 1;
        tail = results.len() - 1;

        *depth -= 1;
    }
}
