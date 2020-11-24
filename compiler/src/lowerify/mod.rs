pub mod compiler;

use crate::lowerify::compiler::Compiler;
use inkwell::context::Context;

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
