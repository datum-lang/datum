extern crate lalrpop;

fn main() {
    lalrpop::process_root().unwrap();

    // todo: thinking in generate code in codebase
    // lalrpop::Configuration::new()
    //     .generate_in_source_tree()
    //     .emit_rerun_directives(true)
    //     .process()
    //     .unwrap();
}
