use clap::{App, Arg};
use compiler::compiler::Compiler;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

fn main() {
    let matches = App::new("solang")
        .version(&*format!("version {}", env!("GIT_HASH")))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("INPUT")
                .help("Charj input files")
                .required(true)
                .multiple(true),
        )
        .get_matches();

    for filename in matches.values_of("INPUT").unwrap() {
        if let Ok(path) = PathBuf::from(filename).canonicalize() {
            let mut contents = String::new();
            let mut f = File::open(&path).unwrap();
            if let Err(e) = f.read_to_string(&mut contents) {
                panic!("failed to read file ‘{}’: {}", filename, e.to_string())
            }

            Compiler::create(&*contents);
        }
    }
}
