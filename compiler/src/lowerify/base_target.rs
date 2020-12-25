use crate::code_object::CodeObject;
use crate::ControlFlowGraph;
use cjc_mir::instruction::ExprKind;
use inkwell::types::BasicTypeEnum;
use inkwell::values::FunctionValue;

pub trait BaseTarget<'a> {
    fn emit_function(&self, sb: &mut CodeObject, cfg: &ControlFlowGraph) {
        let function = self.create_llvm_function(sb, &cfg);
        self.emit_cfg(sb, function, cfg);
    }

    fn create_llvm_function<'func>(
        &self,
        co: &mut CodeObject<'func>,
        cfg: &ControlFlowGraph,
    ) -> FunctionValue<'func> {
        let ret_type = co.context.i32_type();
        let args_types = std::iter::repeat(ret_type)
            .take(cfg.params.len())
            .map(|f| f.into())
            .collect::<Vec<BasicTypeEnum>>();
        let args_types = args_types.as_slice();

        let fn_type = co.context.i32_type().fn_type(args_types, false);

        let func_decl = co.module.add_function(&cfg.name, fn_type, None);
        func_decl
    }

    fn emit_cfg(&self, sb: &mut CodeObject, function: FunctionValue, cfg: &ControlFlowGraph) {
        let bb = sb.context.append_basic_block(function, &cfg.name);
        sb.builder.position_at_end(bb);

        for instr in &cfg.blocks.instructions {
            match instr {
                ExprKind::Var { .. } => {}
                ExprKind::Call { value } => {
                    sb.emit_call(value);
                }
                ExprKind::Print { value } => {
                    sb.emit_print(&"", value);
                }
            }
        }

        sb.emit_void();
    }
}
