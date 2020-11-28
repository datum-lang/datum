use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::targets::TargetTriple;

use crate::Namespace;

pub struct TargetBuilder<'a> {
    pub name: String,
    pub module: Module<'a>,
    builder: Builder<'a>,
    context: &'a Context,
}

impl<'a> TargetBuilder<'a> {
    pub fn new(
        name: &'a str,
        context: &'a Context,
        ns: &'a Namespace,
        filename: &'a str,
    ) -> Self {
        let triple = TargetTriple::create("x86_64");
        let module = context.create_module(&name);

        module.set_triple(&triple);
        module.set_source_file_name(filename);

        TargetBuilder {
            name: name.to_owned(),
            module,
            builder: context.create_builder(),
            context
        }
    }
}