use std::path::Path;

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::{Linkage, Module};
use inkwell::targets::{CodeModel, FileType, RelocMode, TargetTriple};
use inkwell::types::IntType;
use inkwell::values::PointerValue;
use inkwell::{AddressSpace, OptimizationLevel};

use crate::{ControlFlowGraph, Namespace};

#[allow(dead_code)]
#[derive(Debug)]
pub struct CodeObject<'a> {
    pub name: &'a str,
    pub module: Module<'a>,
    pub context: &'a Context,
    pub(crate) builder: Builder<'a>,
    pub cfg: &'a ControlFlowGraph,
}

impl<'a> CodeObject<'a> {
    pub fn new(
        name: &'a str,
        cfg: &'a ControlFlowGraph,
        context: &'a Context,
        filename: &'a str,
        _ns: &'a Namespace,
        target: &str,
    ) -> Self {
        let triple = TargetTriple::create(target);
        let module = context.create_module(&name);

        module.set_triple(&triple);
        module.set_source_file_name(filename);

        CodeObject {
            name: &name,
            module,
            builder: context.create_builder(),
            context,
            cfg,
        }
    }

    pub(crate) fn emit_print(&self, name: &&str, data: &str) -> IntType {
        let i32_type = self.context.i32_type();
        let str_type = self.context.i8_type().ptr_type(AddressSpace::Generic);
        let printf_type = i32_type.fn_type(&[str_type.into()], true);

        let printf = self
            .module
            .add_function("puts", printf_type, Some(Linkage::External));

        let pointer_value = self.emit_global_string(name, data.as_ref(), false);
        self.builder.build_call(printf, &[pointer_value.into()], "");

        i32_type
    }

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

    pub fn emit_void(&mut self) {
        self.builder
            .build_return(Some(&self.context.i32_type().const_zero()));
    }

    pub fn bitcode(&self, path: &Path) {
        self.module.write_bitcode_to_path(path);
    }

    pub fn run_jit(&self) -> i32 {
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
            compiled_fn.call()
        }
    }

    pub fn dump_llvm(&self, path: &Path) -> Result<(), String> {
        if let Err(s) = self.module.print_to_file(path) {
            return Err(s.to_string());
        }

        Ok(())
    }

    pub fn code(&self) {
        let target = inkwell::targets::Target::from_name("wasm").unwrap();
        let target_machine = target
            .create_target_machine(
                &TargetTriple::create("wasm32-unknown-unknown-wasm"),
                "",
                "",
                OptimizationLevel::None,
                RelocMode::Default,
                CodeModel::Default,
            )
            .unwrap();

        match target_machine.write_to_memory_buffer(&self.module, FileType::Object) {
            Ok(out) => {
                let slice = out.as_slice();
                println!("{:?}", slice);
            }
            Err(_) => {}
        }
    }
}
