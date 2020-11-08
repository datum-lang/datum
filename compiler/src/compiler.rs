use std::collections::HashMap;

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::passes::PassManager;
use inkwell::values::{FunctionValue, PointerValue};

use inkwell::types::BasicTypeEnum;
use parser::parse_tree::{
    Expression, ExpressionType, SourceUnit, SourceUnitPart, Statement, StatementType, StructFuncDef,
};
use parser::parser::parse_program;

#[allow(dead_code)]
pub struct Compiler<'a, 'ctx> {
    pub context: &'ctx Context,
    pub builder: &'a Builder<'ctx>,
    pub fpm: &'a PassManager<FunctionValue<'ctx>>,
    pub module: &'a Module<'ctx>,
    pub source_unit: &'a SourceUnit,

    variables: HashMap<String, PointerValue<'ctx>>,
    fn_value_opt: Option<FunctionValue<'ctx>>,
}

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Compiles the specified `Function` in the given `Context` and using the specified `Builder`, `PassManager`, and `Module`.
    pub fn compile(
        context: &'ctx Context,
        builder: &'a Builder<'ctx>,
        pass_manager: &'a PassManager<FunctionValue<'ctx>>,
        module: &'a Module<'ctx>,
        source_unit: &SourceUnit,
    ) {
        let mut compiler = Compiler {
            context: context,
            builder: builder,
            fpm: pass_manager,
            module,
            source_unit,
            fn_value_opt: None,
            variables: HashMap::new(),
        };

        compiler.compile_source();
    }

    fn compile_source(&mut self) {
        for part in self.source_unit.0.iter() {
            use SourceUnitPart::*;
            match part {
                ImportDirective(_) => {}
                MultipleImportDirective(_) => {}
                PackageDirective(_) => {}
                StructFuncDef(fun) => {
                    self.compile_fn(fun);
                }
                FuncDef(_) => {}
                StructDef(_) => {}
            }
        }
    }

    fn compile_fn(
        &mut self,
        fun: &Box<StructFuncDef>,
    ) -> Result<FunctionValue<'ctx>, &'static str> {
        let func = self.compile_prototype(fun)?;
        if fun.body.len() == 0 {
            return Ok(func);
        }

        self.scan_statement(fun.body.as_ref());
        return Ok(func);
    }

    fn scan_statement(&mut self, body: &Vec<Statement>) {
        use StatementType::*;
        for stmt in body {
            match stmt.node {
                Break => {}
                Continue => {}
                If { .. } => {}
                While { .. } => {}
                For { .. } => {}
                Loop => {}
                Assign { .. } => {}
                Variable { .. } => {}
                Return { .. } => {}
                Expression { ref expression } => {
                    self.scan_expression(expression);
                }
            }
        }
    }

    fn scan_expression(&mut self, expression: &Expression) {
        use ExpressionType::*;
        match &expression.node {
            Range { .. } => {}
            BoolOp { .. } => {}
            Binop { .. } => {}
            Unop { .. } => {}
            String { .. } => {}
            Number { .. } => {}
            List { .. } => {}
            Identifier { .. } => {}
            Type { .. } => {}
            Attribute { .. } => {}
            Call { function, args, .. } => {
                println!("{:?} - {:?}", function, args);
            }
            SimpleCompare { .. } => {}
            Compare { .. } => {}
        };
    }

    fn compile_prototype(
        &mut self,
        fun: &Box<StructFuncDef>,
    ) -> Result<FunctionValue<'ctx>, &'static str> {
        let ret_type = self.context.f64_type();
        let args_types = std::iter::repeat(ret_type)
            .take(0)
            .map(|f| f.into())
            .collect::<Vec<BasicTypeEnum>>();
        let args_types = args_types.as_slice();

        let fn_type = self.context.f64_type().fn_type(args_types, false);
        let fn_val = self
            .module
            .add_function(fun.name.name.as_str(), fn_type, None);

        // set arguments names
        // for (i, arg) in fn_val.get_param_iter().enumerate() {
        //     arg.into_float_value().set_name(fun.params[i].as_str());
        // }

        Ok(fn_val)
    }
}

pub fn create_compiler(input: &str) {
    let context = Context::create();
    let module = context.create_module("repl");
    let builder = context.create_builder();

    let fpm = PassManager::create(&module);
    fpm.add_instruction_combining_pass();
    fpm.add_reassociate_pass();
    fpm.add_gvn_pass();
    fpm.add_cfg_simplification_pass();
    fpm.add_basic_alias_analysis_pass();
    fpm.add_promote_memory_to_register_pass();
    fpm.add_instruction_combining_pass();
    fpm.add_reassociate_pass();

    fpm.initialize();

    let parse_ast = parse_program(input);
    match parse_ast {
        Ok(unit) => {
            Compiler::compile(&context, &builder, &fpm, &module, &unit);
        }
        Err(_) => {}
    }
}

#[cfg(test)]
mod test {
    use crate::compiler::create_compiler;

    #[test]
    #[rustfmt::skip]
    fn init_parser() {
        create_compiler("default$main(string name) {fmt.println(\"hello,world\")}");
    }
}
