use std::path::Path;

use inkwell::context::Context;

use crate::lowerify::classic_target::ClassicTarget;
use crate::lowerify::wasm_target::WasmTarget;
use crate::Namespace;

pub mod base_target;
pub mod classic_target;
pub mod code_object;
pub mod wasm_target;

lazy_static::lazy_static! {
    static ref LLVM_INIT: () = {
        inkwell::targets::Target::initialize_webassembly(&Default::default());
    };
}

pub fn codegen(ns: &mut Namespace, target: &str) {
    for no in 0..ns.cfgs.len() {
        let cfg = &ns.cfgs[no];

        let filename = ns.files[0].clone();
        let context = Context::create();

        match target {
            "jit" => {
                let obj = ClassicTarget::build(&filename, cfg, &context, ns);
                obj.run_jit();
            }
            "wasm" => {
                // lazy_static::initialize(&LLVM_INIT);
                let obj = WasmTarget::build(&filename, cfg, &context, ns);
                obj.code();
            }
            "llvm" => {
                let obj = ClassicTarget::build(&filename, cfg, &context, ns);
                let name = format!("{}.ll", &cfg.name);
                match obj.dump_llvm(Path::new(&name)) {
                    Ok(_) => {
                        println!("dump llvm success: {:?}", name);
                    }
                    Err(_) => {
                        panic!("dump llvm failured: {:?}", name);
                    }
                }
            }
            &_ => {
                let obj = ClassicTarget::build(&filename, cfg, &context, ns);
                let name = format!("{}.bc", &cfg.name);
                obj.bitcode(Path::new(&name));
            }
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    #[rustfmt::skip]
    fn init_parser() {}
}
