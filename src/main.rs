use std::fs::read_dir;
use std::io::BufWriter;
use std::io::Stderr;
use std::io::Write;
use std::path::PathBuf;
use std::process::exit;

use lsdep::settings::Settings;

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

    let mut working_list = Vec::with_capacity(depth.pow(2) as usize);
    let mut result_list: Vec<PathBuf> = Vec::with_capacity(depth.pow(2) as usize);

    working_list.push(settings.base);

    walk_dirs(&settings.depth, &mut working_list, &mut err_writer, &mut result_list);

    err_writer.flush().unwrap();
}


fn walk_dirs(d: &u32, working_dirs: &mut Vec<PathBuf>, err_stream: &mut BufWriter<Stderr>, result_dirs: &mut Vec<PathBuf>) {
    let mut head = 0;
    let mut tail = working_dirs.len() - 1;
    let mut depth = d.clone();

    while depth > 0 {
        let prev_len = working_dirs.len();

        for i in head..=tail {
            let path: &PathBuf = working_dirs.get(i).unwrap();
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
                    if depth == 1 {
                        result_dirs.push(entry.path());
                    }
                    working_dirs.push(path);
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
}

