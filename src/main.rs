use std::io::Write;

use lsdep::settings::Settings;

fn main() {
    let settings = Settings::from_args(std::env::args());
    write!(std::io::stdout(), "{settings}\n").unwrap();
    let mut result_list: Vec<std::path::PathBuf> = Vec::with_capacity((10 * settings.depth()) as usize);
    lsdep::walk_dirs(&settings, &mut result_list);
}
