use crate::base_target::BaseTarget;
use crate::code_object::CodeObject;
use crate::{ControlFlowGraph, Namespace};
use inkwell::context::Context;

pub struct WasmTarget {}

impl WasmTarget {
    pub fn build<'a>(
        filename: &'a String,
        context: &'a Context,
        ns: &'a Namespace,
    ) -> CodeObject<'a> {
        let target = WasmTarget {};

        let wasm_target = "wasm32-unknown-unknown-wasm";
        let mut structure = CodeObject::new(context, filename, ns, wasm_target);
        for cfg in &ns.cfgs {
            target.emit_function(&mut structure, &cfg);
        }

        structure
    }
}

impl<'a> BaseTarget<'a> for WasmTarget {}
