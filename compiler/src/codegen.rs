use inkwell::execution_engine::ExecutionEngine;
use inkwell::module::Module;
use inkwell::OptimizationLevel;

#[allow(dead_code)]
struct CodeGen<'ctx> {
    execution_engine: ExecutionEngine<'ctx>,
}

#[allow(dead_code)]
impl<'ctx> CodeGen<'ctx> {
    pub fn new(module: &'ctx Module) -> Self {
        let execution_engine = module
            .create_jit_execution_engine(OptimizationLevel::None)
            .unwrap();

        CodeGen { execution_engine }
    }
}
