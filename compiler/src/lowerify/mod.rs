pub mod compiler;
use crate::neat::Namespace;
use inkwell::context::Context;
use cjc_parser::parser::parse_program;
use crate::lowerify::compiler::Compiler;

pub fn compile_program(input: &str, filename: &str) -> Result<String, ()> {
    let mut namespace = Namespace::new();

    let context = Context::create();
    let module_name = filename.replace(".cj", "");

    let module = context.create_module(&module_name);
    let builder = context.create_builder();

    namespace.files.push(filename.to_string());

    let parse_ast = parse_program(input);
    match parse_ast {
        Ok(unit) => {
            // todo: set target
            // let triple = TargetTriple::create(ns.target.llvm_target_triple());

            // todo: load stdlib
            // let intr = Compiler::load_stdlib(&context);
            // module.link_in_module(intr).unwrap();

            let compiler = Compiler::compile(&context, &builder, &module, &unit, &mut namespace);
            Ok(compiler.module.print_to_string().to_string())
        }
        Err(_) => Err(()),
    }
}

#[cfg(test)]
mod test {
    use crate::lowerify::compile_program;

    #[test]
    #[rustfmt::skip]
    fn init_parser() {
        let result = compile_program(
            "default$main() {println(\"hello,world\")}",
            "hello.cj",
        );
    }
}