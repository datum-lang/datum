use inkwell::context::Context;

use crate::lowerify::code_object::CodeObject;
use crate::{ControlFlowGraph, Namespace};
use cjc_mir::instruction::ExprKind;
use inkwell::types::BasicTypeEnum;
use inkwell::values::FunctionValue;

pub struct ClassicTarget {}

impl ClassicTarget {
    pub fn build<'a>(
        filename: &'a String,
        cfg: &'a ControlFlowGraph,
        context: &'a Context,
        ns: &'a Namespace,
    ) -> CodeObject<'a> {
        let target = ClassicTarget {};

        let mut structure = CodeObject::new(filename, cfg, context, "", ns);

        target.emit_function(&mut structure);

        println!("{:?}", structure.module.print_to_string());

        structure.run_jit();

        structure
    }

    pub fn emit_function(mut self, sb: &mut CodeObject) {
        let funtion = ClassicTarget::create_llvm_function(sb);
        self.emit_cfg(sb, funtion);
    }

    fn create_llvm_function<'func>(sb: &mut CodeObject<'func>) -> FunctionValue<'func> {
        let ret_type = sb.context.i32_type();
        let args_types = std::iter::repeat(ret_type)
            .take(sb.cfg.params.len())
            .map(|f| f.into())
            .collect::<Vec<BasicTypeEnum>>();
        let args_types = args_types.as_slice();

        let fn_type = sb.context.i32_type().fn_type(args_types, false);

        let func_decl = sb.module.add_function(&sb.cfg.name, fn_type, None);
        func_decl
    }

    pub fn emit_cfg(mut self, sb: &mut CodeObject, function: FunctionValue) {
        let bb = sb.context.append_basic_block(function, &sb.cfg.name);
        sb.builder.position_at_end(bb);

        for instr in &sb.cfg.basic_block.instructions {
            match instr {
                ExprKind::Var { .. } => {}
                ExprKind::Call => {}
                ExprKind::Print { value } => {
                    sb.emit_print(&"", value);
                }
            }
        }

        sb.emit_void();
    }
}
