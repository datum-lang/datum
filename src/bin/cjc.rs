use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use clap::{App, Arg};

use compiler::{codegen, process_string};

fn main() {
    let matches = App::new("charj")
        .version(&*format!("version {}", env!("GIT_HASH")))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("INPUT")
                .help("Charj input files")
                .required(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("TARGET")
                .help("Output target")
                .multiple(true)
                .last(true)
                .default_value("jit"),
        )
        .get_matches();

    for filename in matches.values_of("INPUT").unwrap() {
        if let Ok(path) = PathBuf::from(filename).canonicalize() {
            let mut contents = String::new();
            let mut f = File::open(&path).unwrap();
            if let Err(e) = f.read_to_string(&mut contents) {
                panic!("failed to read file ‘{}’: {}", filename, e.to_string())
            }

            let mut ns = process_string(&*contents, filename);
            if let Some("jit") = matches.value_of("TARGET") {
                codegen(&mut ns, "jit");
            } else if let Some("wasm") = matches.value_of("TARGET") {
                codegen(&mut ns, "wasm");
            } else {
                panic!("not support target{:?}", matches.value_of("TARGET"));
            }
        } else {
            panic!("lost file: {:?}", filename);
        }
    }
}
