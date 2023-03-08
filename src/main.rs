use std::fs::read_dir;
use std::io::stderr;
use std::io::stdout;
use std::io::BufWriter;
use std::io::Write;
use std::path::PathBuf;

use lsdep::settings::Settings;

fn main() {
    let settings = Settings::from_args(std::env::args());
    write!(stdout(), "{settings}\n").unwrap();
    let mut result_list: Vec<PathBuf> = Vec::with_capacity((10 * settings.depth()) as usize);
    walk_dirs(&settings, &mut result_list);
}

fn walk_dirs(settings: &Settings, result_dirs: &mut Vec<PathBuf>) {
    let mut depth = settings.depth();
    let capacity: usize = if depth >= 20 { 2usize.pow(20) } else { 2usize.pow((depth + 1) as u32) };

    let mut working_dirs: Vec<PathBuf> = Vec::with_capacity(capacity);

    working_dirs.push(settings.base());

    let mut err_stream = BufWriter::new(stderr());

    let mut head = 0;
    let mut tail = working_dirs.len() - 1;

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

    err_stream.flush().unwrap();
}
