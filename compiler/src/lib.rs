use inkwell::context::Context;

use cjc_codegen::ControlFlowGraph;
use cjc_parser::parser::parse_program;

use crate::compiler::Compiler;
use inkwell::memory_buffer::MemoryBuffer;
use inkwell::module::Module;
use inkwell::targets::TargetTriple;

pub mod builtin;
pub mod compiler;
pub mod namespace;
pub mod symbol_table;

pub fn compile_program(input: &str, filename: &str) -> Result<String, ()> {
    let context = Context::create();
    let module_name = filename.replace(".cj", "");

    let module = context.create_module(&module_name);
    let builder = context.create_builder();

    let mut all_cfg: Vec<ControlFlowGraph> = Vec::new();
    let parse_ast = parse_program(input);
    match parse_ast {
        Ok(unit) => {
            // todo: set target
            // let triple = TargetTriple::create(ns.target.llvm_target_triple());

            // todo: load stdlib
            // let intr = Compiler::load_stdlib(&context);
            // module.link_in_module(intr).unwrap();

            let compiler = Compiler::compile(&context, &builder, &module, &unit);
            compiler.run_jit();
            Ok(compiler.module.print_to_string().to_string())
        }
        Err(_) => Err(()),
    }
}

#[cfg(test)]
mod test {
    use crate::compile_program;

    #[test]
    #[rustfmt::skip]
    fn init_parser() {
        let result = compile_program(
            "default$main() {println(\"hello,world\")}",
            "hello.cj"
        );

        assert!(result.is_ok());
        println!("{}", result.unwrap());
    }
}
