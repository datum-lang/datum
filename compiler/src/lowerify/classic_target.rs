use inkwell::context::Context;

use crate::lowerify::code_object::CodeObject;
use crate::{ControlFlowGraph, Namespace};
use crate::base_target::BaseTarget;

pub struct ClassicTarget {}

impl ClassicTarget {
    pub fn build<'a>(
        filename: &'a String,
        cfg: &'a ControlFlowGraph,
        context: &'a Context,
        ns: &'a Namespace,
    ) -> CodeObject<'a> {
        let target = ClassicTarget {};

        let mut structure = CodeObject::new(&*cfg.name, cfg, context, filename, ns, "x86_64");
        target.emit_function(&mut structure);

        structure
    }
}

impl<'a> BaseTarget<'a> for ClassicTarget {}
