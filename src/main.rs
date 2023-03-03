use std::fs::read_dir;
use std::io::BufWriter;
use std::io::Stderr;
use std::io::Stdout;
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

    let depth = args_iter.next();

    if depth.is_none() {
        write!(err_writer, "no depth is given, use default 1\n").unwrap();
    }

    let mut depth: u32 = match depth {
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

    let base = PathBuf::from(base_dir.as_ref().unwrap());
    let mut results = Vec::new();

    results.push(base);

    // 1 MB capacity
    let mut out_stream = BufWriter::with_capacity(10_000_000, std::io::stdout());

    walk_dirs(&mut depth, &mut results, &mut err_writer, &mut out_stream);

    err_writer.flush().unwrap();
    out_stream.flush().unwrap();
}

fn walk_dirs(depth: &mut u32, results: &mut Vec<PathBuf>, err_stream: &mut BufWriter<Stderr>, out_stream: &mut BufWriter<Stdout>) {
    let mut head = 0;
    let mut tail = results.len() - 1;

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

                // is_dir will traverse soft link, tested this!
                // check for symlink is necessary
                if path.is_dir() && !path.is_symlink() {
                    write!(out_stream, "{}\n", &path.display()).unwrap();
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
