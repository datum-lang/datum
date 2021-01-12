use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use clap::{App, Arg, ArgMatches};

use compiler::{codegen, process_string, CodegenResult};

mod languageserver;

fn main() {
    let matches = App::new("datum")
        .version(&*format!("version {}", env!("GIT_HASH")))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("INPUT")
                .help("Charj input files")
                .required(true)
                .conflicts_with("LANGUAGESERVER")
                .multiple(true),
        )
        .arg(
            Arg::with_name("TARGET")
                .help("Output target")
                .multiple(true)
                .last(true)
                .default_value("jit"),
        )
        .arg(
            Arg::with_name("LANGUAGESERVER")
                .help("Start language server")
                .conflicts_with_all(&["INPUT"])
                .long("language-server"),
        )
        .get_matches();

    if matches.is_present("LANGUAGESERVER") {
        languageserver::start_server();
    }

    // todo: split input handler and namespace actions
    for filename in matches.values_of("INPUT").unwrap() {
        process_filename(filename, &matches);
    }
}

pub fn process_filename(filename: &str, matches: &ArgMatches) {
    if let Err(_) = PathBuf::from(filename).canonicalize() {
        panic!("lost file: {:?}", filename);
    }

    let path = PathBuf::from(filename).canonicalize().unwrap();
    let mut contents = String::new();
    let mut f = File::open(&path).unwrap();
    if let Err(e) = f.read_to_string(&mut contents) {
        panic!("failed to read file ‘{}’: {}", filename, e.to_string())
    }

    let mut ns = process_string(&*contents, filename);
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
