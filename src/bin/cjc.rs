use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use clap::{App, Arg};

use compiler::{codegen, process_string, CodegenResult};

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

    let mut namespaces = vec![];

    // todo: split input handler and namespace actions
    for filename in matches.values_of("INPUT").unwrap() {
        if let Ok(path) = PathBuf::from(filename).canonicalize() {
            let mut contents = String::new();
            let mut f = File::open(&path).unwrap();
            if let Err(e) = f.read_to_string(&mut contents) {
                panic!("failed to read file ‘{}’: {}", filename, e.to_string())
            }

            let ns = process_string(&*contents, filename);
            namespaces.push(ns);
        } else {
            panic!("lost file: {:?}", filename);
        }
    }

    // todo: refactor using same namespaces
    for mut ns in namespaces {
        match matches.value_of("TARGET") {
            Some("jit") => {
                codegen(&mut ns, "jit");
            }
            Some("wasm") => {
                let result = codegen(&mut ns, "wasm");
                let mut file = File::create("out.wasm").unwrap();
                match &result[0] {
                    CodegenResult::Wasm { code } => file.write_all(code).unwrap(),
                    _ => {}
                }
            }
            _ => {
                panic!("not support target{:?}", matches.value_of("TARGET"));
            }
        }
    }
}
