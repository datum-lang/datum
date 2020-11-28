use inkwell::context::Context;

use crate::{Namespace, ControlFlowGraph};
use crate::lowerify::struct_builder::StructBuilder;

pub struct ClassicTarget {}

impl ClassicTarget {
    pub fn build<'a>(
        filename: &String,
        cfg: &'a ControlFlowGraph,
        context: &'a Context,
        ns: &'a Namespace,
    ) -> Self {
        let target = ClassicTarget {};

        let mut structure = StructBuilder::new(filename, cfg, context, "", ns);

        target.emit_functions(&mut structure);

        Self {}
    }

    pub fn emit_functions(mut self, sb: &mut StructBuilder) {
        self.emit_cfg(sb);
    }

    pub fn emit_cfg(mut self, sb: &mut StructBuilder) {}
}

