use std::collections::HashMap;
use std::path::Path;

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Linkage;
use inkwell::module::Module;
use inkwell::types::{BasicTypeEnum, IntType};
use inkwell::values::{BasicValue, FunctionValue, PointerValue};
use inkwell::{AddressSpace, OptimizationLevel};

use codegen::instruction::{Constant, Instruction};
use parser::location::Location;
use parser::parse_tree::{
    Argument, Expression, ExpressionType, SourceUnit, SourceUnitPart, Statement, StatementType,
    StructFuncDef,
};

#[allow(dead_code)]
pub struct Compiler<'a, 'ctx> {
    pub context: &'ctx Context,
    pub builder: &'a Builder<'ctx>,
    pub module: &'a Module<'ctx>,
    pub source_unit: &'a SourceUnit,

    variables: HashMap<String, PointerValue<'ctx>>,
    fn_value_opt: Option<FunctionValue<'ctx>>,
    current_source_location: Location,
}

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Gets a defined function given its name.
    #[inline]
    fn get_function(&self, name: &str) -> Option<FunctionValue<'ctx>> {
        self.module.get_function(name)
    }

    pub fn load_stdlib(_context: &Context) {
        // todo: thinking in stdlib
    }

    /// Compiles the specified `Function` in the given `Context` and using the specified `Builder`, `PassManager`, and `Module`.
    pub fn compile(
        context: &'ctx Context,
        builder: &'a Builder<'ctx>,
        module: &'a Module<'ctx>,
        source_unit: &'a SourceUnit,
    ) -> Compiler<'a, 'ctx> {
        // todo: set target
        // let triple = TargetTriple::create(ns.target.llvm_target_triple());

        // todo: load stdlib
        // let intr = Compiler::load_stdlib(&context);
        // module.link_in_module(intr).unwrap();

        let mut compiler = Compiler {
            context,
            builder,
            module,
            source_unit,
            fn_value_opt: None,
            variables: HashMap::new(),
            current_source_location: Default::default(),
        };

        compiler.compile_source();
        compiler
    }

    fn compile_source(&mut self) {
        for part in self.source_unit.0.iter() {
            use SourceUnitPart::*;
            match part {
                ImportDirective(_) => {}
                MultipleImportDirective(_) => {}
                PackageDirective(_) => {}
                StructFuncDef(fun) => {
                    let _result = self.compile_struct_fn(fun);
                }
                FuncDef(_) => {}
                StructDef(_) => {}
            }
        }
    }

    fn compile_struct_fn(
        &mut self,
        func_def: &Box<StructFuncDef>,
    ) -> Result<FunctionValue<'ctx>, &'static str> {
        let func = self.compile_prototype(func_def)?;
        if func_def.body.len() == 0 {
            return Ok(func);
        }
        let entry = self
            .context
            .append_basic_block(func, func_def.name.name.as_str());

        self.builder.position_at_end(entry);

        // update fn field
        self.fn_value_opt = Some(func);

        // build variables map
        self.variables.reserve(func_def.params.len());
        for (i, arg) in func.get_param_iter().enumerate() {
            let arg_name = func_def.params[i].1.as_ref().unwrap().get_name();
            let alloca = self.create_entry_block_alloca(&*arg_name);

            self.builder.build_store(alloca, arg);
        }

        self.compile_statement(func_def.body.as_ref());

        if func_def.returns.is_none() {
            self.emit_void();
        }

        return Ok(func);
    }

    fn emit_void(&mut self) {
        self.builder
            .build_return(Some(&self.context.i32_type().const_zero()));
    }

    fn compile_statement(&mut self, body: &Vec<Statement>) {
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
                    self.compile_expression(expression);
                }
            }
        }
    }

    fn compile_expression(&mut self, expression: &Expression) {
        use ExpressionType::*;
        // println!("{:?}", expression.node);
        match &expression.node {
            Range { .. } => {}
            BoolOp { .. } => {}
            Binop { .. } => {}
            Unop { .. } => {}
            String { value } => {
                self.emit(Instruction::LoadConst {
                    value: Constant::String {
                        value: value.to_string(),
                    },
                });
            }
            Number { .. } => {}
            List { .. } => {}
            Identifier { name: _ } => {
                // self.emit();
            }
            Type { .. } => {}
            Attribute { value, name: _ } => {
                self.compile_expression(value);
            }
            Call { function, args, .. } => self.function_call_expr(function, args),
            SimpleCompare { .. } => {}
            Compare { .. } => {}
        };
    }

    fn emit(&mut self, _instruction: Instruction) {}

    fn function_call_expr(&mut self, expr: &Box<Expression>, args: &Vec<Argument>) {
        self.compile_expression(expr);

        // match self.get_function("main") {
        //     None => {}
        //     Some(_func) => {
        //         for x in args.iter() {
        //             self.compile_expression(&x.expr);
        //         }
        //
        //         self.emit_print(&"hello", "hello, world!\n");
        //     }
        // };
    }

    fn compile_prototype(
        &mut self,
        fun: &Box<StructFuncDef>,
    ) -> Result<FunctionValue<'ctx>, &'static str> {
        let ret_type = self.context.i32_type();
        let args_types = std::iter::repeat(ret_type)
            .take(fun.params.len())
            .map(|f| f.into())
            .collect::<Vec<BasicTypeEnum>>();
        let args_types = args_types.as_slice();

        let fn_type = self.context.i32_type().fn_type(args_types, false);
        let fn_val = self
            .module
            .add_function(fun.name.name.as_str(), fn_type, None);

        // set arguments names
        for (i, arg) in fn_val.get_param_iter().enumerate() {
            let x = &*fun.params[i].1.as_ref().unwrap().get_name();
            arg.into_int_value().set_name(x);
        }

        Ok(fn_val)
    }

    fn emit_print(&self, name: &&str, data: &str) -> IntType {
        let i32_type = self.context.i32_type();
        let str_type = self.context.i8_type().ptr_type(AddressSpace::Generic);
        let printf_type = i32_type.fn_type(&[str_type.into()], true);

        // `printf` is same to `puts`
        let printf = self
            .module
            .add_function("puts", printf_type, Some(Linkage::External));

        let pointer_value = self.emit_global_string(name, data.as_ref(), false);
        self.builder.build_call(printf, &[pointer_value.into()], "");

        i32_type
    }

    /// Creates a new stack allocation instruction in the entry block of the function.
    fn create_entry_block_alloca(&self, name: &str) -> PointerValue<'ctx> {
        let builder = self.context.create_builder();

        let entry = self.fn_value().get_first_basic_block().unwrap();

        match entry.get_first_instruction() {
            Some(first_instr) => builder.position_before(&first_instr),
            None => builder.position_at_end(entry),
        }

        builder.build_alloca(self.context.f64_type(), name)
    }

    /// Returns the `FunctionValue` representing the function being compiled.
    #[inline]
    fn fn_value(&self) -> FunctionValue<'ctx> {
        self.fn_value_opt.unwrap()
    }

    /// Creates global string in the llvm module with initializer
    ///
    fn emit_global_string(&self, name: &str, data: &[u8], constant: bool) -> PointerValue<'a> {
        let ty = self.context.i8_type().array_type(data.len() as u32);

        let gv = self
            .module
            .add_global(ty, Some(AddressSpace::Generic), name);

        gv.set_linkage(Linkage::Internal);

        gv.set_initializer(&self.context.const_string(data, false));

        if constant {
            gv.set_constant(true);
            gv.set_unnamed_addr(true);
        }

        self.builder.build_pointer_cast(
            gv.as_pointer_value(),
            self.context.i8_type().ptr_type(AddressSpace::Generic),
            name,
        )
    }

    pub fn bitcode(&self, path: &Path) {
        self.module.write_bitcode_to_path(path);
    }

    pub fn dump_llvm(&self, path: &Path) -> Result<(), String> {
        if let Err(s) = self.module.print_to_file(path) {
            return Err(s.to_string());
        }

        Ok(())
    }

    pub fn run_jit(&self) {
        // todo: verify
        self.module.get_function("main").unwrap().verify(true);

        let ee = self
            .module
            .create_jit_execution_engine(OptimizationLevel::None)
            .unwrap();
        let maybe_fn = unsafe {
            // todo: thinking in return of main func
            ee.get_function::<unsafe extern "C" fn() -> i32>("main")
        };

        let compiled_fn = match maybe_fn {
            Ok(f) => f,
            Err(err) => {
                panic!("{:?}", err);
            }
        };

        unsafe {
            compiled_fn.call();
        }
    }
}
