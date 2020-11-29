use crate::base_target::BaseTarget;
use crate::code_object::CodeObject;
use crate::{ControlFlowGraph, Namespace};
use inkwell::context::Context;

pub struct WasmTarget {}

impl WasmTarget {
    pub fn build<'a>(
        filename: &'a String,
        cfg: &'a ControlFlowGraph,
        context: &'a Context,
        ns: &'a Namespace,
    ) -> CodeObject<'a> {
        let target = WasmTarget {};

        let mut structure = CodeObject::new(
            &*cfg.name,
            cfg,
            context,
            filename,
            ns,
            "wasm32-unknown-unknown-wasm",
        );
        target.emit_function(&mut structure);

        structure
    }
}

impl<'a> BaseTarget<'a> for WasmTarget {}
