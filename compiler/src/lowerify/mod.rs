use inkwell::context::Context;

use crate::lowerify::classic_target::ClassicTarget;
use crate::lowerify::compiler::Compiler;
use crate::Namespace;
use std::path::Path;

pub mod classic_target;
pub mod compiler;
pub mod struct_builder;
pub mod wasm_target;

pub fn codegen(ns: &mut Namespace) {
    for no in 0..ns.cfgs.len() {
        let cfg = &ns.cfgs[no];

        let filename = ns.files[0].clone();
        let context = Context::create();

        let compiler = ClassicTarget::build(&filename, cfg, &context, ns);
        compiler.bitcode(Path::new("main.cjc"));
    }
}

pub fn compile_program(_input: &str, filename: &str) -> Result<String, ()> {
    let context = Context::create();
    let module_name = filename.replace(".cj", "");

    let module = context.create_module(&module_name);
    let builder = context.create_builder();

    let compiler = Compiler::compile(&context, &builder, &module);
    Ok(compiler.module.print_to_string().to_string())
}

#[cfg(test)]
mod test {
    use crate::lowerify::compile_program;

    #[test]
    #[rustfmt::skip]
    fn init_parser() {
        let _result = compile_program(
            "default$main() {println(\"hello,world\")}",
            "hello.cj",
        );
    }
}
