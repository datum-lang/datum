use inkwell::context::Context;

use crate::{Namespace, ControlFlowGraph};
use crate::lowerify::struct_builder::StructBuilder;
use cjc_mir::instruction::ExprKind;
use inkwell::module::Linkage;
use inkwell::types::BasicTypeEnum;
use inkwell::values::FunctionValue;

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

        target.emit_function(&mut structure);

        Self {}
    }

    pub fn emit_function(mut self, sb: &mut StructBuilder) {
        let ret_type = sb.context.i32_type();
        let args_types = std::iter::repeat(ret_type)
            .take(sb.cfg.params.len())
            .map(|f| f.into())
            .collect::<Vec<BasicTypeEnum>>();
        let args_types = args_types.as_slice();

        let fn_type = sb.context.i32_type().fn_type(args_types, false);

        let func_decl =
            sb
                .module
                .add_function(&sb.cfg.name, fn_type, None);

        self.emit_cfg(sb, func_decl);
    }

    pub fn emit_cfg(mut self, sb: &mut StructBuilder, function: FunctionValue) {
        let bb = sb.context.append_basic_block(function, &sb.cfg.name);
        sb.builder.position_at_end(bb);

        for instr in &sb.cfg.basic_block.instructions {
            match instr {
                ExprKind::Var { .. } => {}
                ExprKind::Call => {}
                ExprKind::Print { .. } => {}
            }
        }
    }
}

