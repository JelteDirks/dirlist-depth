use std::io::Write;
use std::io::BufWriter;
use std::process::exit;

fn main() {
    let mut args_iter = std::env::args().skip(1);
    let mut err_writer = BufWriter::new(std::io::stderr());
    let mut out_writer = BufWriter::new(std::io::stdout());

    let base_dir = args_iter.next();

    if base_dir.is_none() {
        write!(err_writer, "no base directory given").unwrap();
        err_writer.flush().unwrap();
        exit(1);
    }

    let depth = args_iter.next();

    if depth.is_none() {
        write!(err_writer, "no depth is given, use default 1").unwrap();
    }

    let depth: u8 = match depth {
        Some(d) => {
            let parsed = d.parse::<u8>();
            if parsed.is_err() {
                write!(err_writer, "problem parsing depth").unwrap();
                exit(2);
            }
            parsed.unwrap()
        },
        None => 1,
    };


    write!(out_writer, "checking {} with depth {}", base_dir.unwrap(), depth).unwrap();

    out_writer.flush().unwrap();
    err_writer.flush().unwrap();
}

