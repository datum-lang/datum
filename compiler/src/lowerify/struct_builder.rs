use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::targets::TargetTriple;

use crate::{Namespace, ControlFlowGraph};

#[allow(dead_code)]
pub struct StructBuilder<'a> {
    pub name: String,
    pub module: Module<'a>,
    builder: Builder<'a>,
    context: &'a Context,
    cfg: &'a ControlFlowGraph,
}

impl<'a> StructBuilder<'a> {
    pub fn new(
        name: &'a str,
        cfg: &'a ControlFlowGraph,
        context: &'a Context,
        filename: &'a str,
        _ns: &'a Namespace,
    ) -> Self {
        let triple = TargetTriple::create("x86_64");
        let module = context.create_module(&name);

        module.set_triple(&triple);
        module.set_source_file_name(filename);

        StructBuilder {
            name: name.to_owned(),
            module,
            builder: context.create_builder(),
            context,
            cfg
        }
    }
}