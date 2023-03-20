# lsdep
I created this project to learn more about the Rust. While making this little
cli, I am reading the second edition of 'The Rust Programming Language' as well
as 'Rust for Rustaceons' and making changes as I go along. For this reason I 
will not be documenting very elaborately. If someone somehow finds some genuine
interest, I might do a better job at that. The documentation might not be up-to-date
for aforementioned reasons.

I won't publish it to cargo (yet?) so everything is based on running it with cargo.

The program is fairly simple:
```zsh
cargo run -- [DIR] [DEPTH]
```

## dir
The directory you want as a base directory. The default is $PWD.

## depth
The depth to which to recurse to. This is an exact depth and only directories
with this exact depth relative to the base directory will be printed.


Example running
```zsh
cargo run -- $PWD 2
```
in the directory
```text
.
├── Cargo.lock
├── Cargo.toml
├── README.md
├── src
│   ├── lib.rs
│   └── main.rs
└── target
    ├── CACHEDIR.TAG
    ├── debug
    └── release

4 directories, 6 files
```
will produce the output:
```text
check /Users/jelte/personal/lsdep with depth 2
/Users/jelte/personal/lsdep/target/release
/Users/jelte/personal/lsdep/target/debug
/Users/jelte/personal/lsdep/.git/objects
/Users/jelte/personal/lsdep/.git/info
/Users/jelte/personal/lsdep/.git/logs
/Users/jelte/personal/lsdep/.git/hooks
/Users/jelte/personal/lsdep/.git/refs
```
