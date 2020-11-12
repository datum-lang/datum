use clap::{App, Arg};

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
        println!("{:?}", filename);
    }
}
