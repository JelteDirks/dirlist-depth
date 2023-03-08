pub mod settings {
    use std::{process::exit, path::PathBuf, fs, io::stderr};
    use std::io::Write;

    pub struct Settings {
        pub depth: u32,
        pub base: PathBuf,
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

        pub fn set_depth(&mut self, depth: u32) {
            self.depth = depth;
        }
    }
}
