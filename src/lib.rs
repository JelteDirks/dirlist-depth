pub mod settings {
    use std::{process::exit, path::PathBuf, fs, io::stderr};
    use std::io::Write;

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

        pub fn get_base_clone(&self) -> PathBuf {
            return self.base.clone();
        }
    }
}
