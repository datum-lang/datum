use crate::code_object::CodeObject;
use cjc_mir::instruction::ExprKind;
use inkwell::types::BasicTypeEnum;
use inkwell::values::FunctionValue;

pub trait BaseTarget<'a> {
    fn emit_function(&self, sb: &mut CodeObject) {
        let function = self.create_llvm_function(sb);
        self.emit_cfg(sb, function);
    }

    fn create_llvm_function<'func>(&self, sb: &mut CodeObject<'func>) -> FunctionValue<'func> {
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

    fn emit_cfg(&self, sb: &mut CodeObject, function: FunctionValue) {
        let bb = sb.context.append_basic_block(function, &sb.cfg.name);
        sb.builder.position_at_end(bb);

        for instr in &sb.cfg.blocks.instructions {
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
