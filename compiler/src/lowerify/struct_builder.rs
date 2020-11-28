use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::{Module, Linkage};
use inkwell::targets::TargetTriple;

use crate::{Namespace, ControlFlowGraph};
use inkwell::values::PointerValue;
use inkwell::AddressSpace;
use inkwell::types::IntType;

#[allow(dead_code)]
#[derive(Debug)]
pub struct StructBuilder<'a> {
    pub name: String,
    pub module: Module<'a>,
    pub context: &'a Context,
    pub(crate) builder: Builder<'a>,
    pub cfg: &'a ControlFlowGraph,
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
            cfg,
        }
    }

    pub(crate) fn emit_print(&self, name: &&str, data: &str) -> IntType {
        let i32_type = self.context.i32_type();
        let str_type = self.context.i8_type().ptr_type(AddressSpace::Generic);
        let printf_type = i32_type.fn_type(&[str_type.into()], true);

        let printf = self
            .module
            .add_function("puts", printf_type, Some(Linkage::External));

        let pointer_value = self.emit_global_string(name, data.as_ref(), false);
        self.builder.build_call(printf, &[pointer_value.into()], "");

        i32_type
    }

    fn emit_global_string(&self, name: &str, data: &[u8], constant: bool) -> PointerValue<'a> {
        let ty = self.context.i8_type().array_type(data.len() as u32);

        let gv = self
            .module
            .add_global(ty, Some(AddressSpace::Generic), name);

        gv.set_linkage(Linkage::Internal);

        gv.set_initializer(&self.context.const_string(data, false));

        if constant {
            gv.set_constant(true);
            gv.set_unnamed_addr(true);
        }

        self.builder.build_pointer_cast(
            gv.as_pointer_value(),
            self.context.i8_type().ptr_type(AddressSpace::Generic),
            name,
        )
    }
}