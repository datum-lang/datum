extern crate lalrpop;

fn main() {
    let _result = lalrpop::Configuration::new()
        .always_use_colors()
        .process_current_dir();

    // todo: thinking in generate code in codebase
    // lalrpop::Configuration::new()
    //     .generate_in_source_tree()
    //     .emit_rerun_directives(true)
    //     .process()
    //     .unwrap();
}
