use std::fs::read_dir;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process::exit;

fn main() {
    let mut args_iter = std::env::args().skip(1);
    let mut err_writer = BufWriter::new(std::io::stderr());
    let mut out_writer = BufWriter::new(std::io::stdout());

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

    let mut depth: u8 = match depth {
        Some(d) => {
            let parsed = d.parse::<u8>();
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

    walk_dirs(&mut depth, &mut results);

    for a in results {
        write!(out_writer, "{:?}\n", a).unwrap();
    }

    out_writer.flush().unwrap();
    err_writer.flush().unwrap();
}

fn walk_dirs(depth: &mut u8, results: &mut Vec<PathBuf>) {
    let mut head = 0;
    let mut tail = 0;

    while *depth > 0 {
        let prev_len = results.len();

        for i in head..=tail {
            let path: &PathBuf = results.get(i).unwrap();

            for entry_res in read_dir(path).unwrap() {
                let entry = entry_res.unwrap();
                let path = entry.path();

                if path.is_dir() {
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
