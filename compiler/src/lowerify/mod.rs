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


#[derive(PartialEq, Clone, Debug)]
pub enum CodegenResult {
    Jit { exit_code: i32 },
    Wasm { code: Vec<u8> },
    LLVM { value: String },
    BitCode,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum CharjTarget {
    Generic,
    WASM,
}

impl CharjTarget {
    #[allow(dead_code)]
    fn llvm_target_name(&self) -> &'static str {
        return match self {
            CharjTarget::Generic => "generic",
            CharjTarget::WASM => "wasm",
        };
    }

    #[allow(dead_code)]
    fn llvm_target_triple(&self) -> &'static str {
        return match self {
            CharjTarget::Generic => "x86_64",
            CharjTarget::WASM => "wasm32-unknown-unknown-wasm",
        };
    }
}

pub fn codegen(ns: &mut Namespace, target: &str) -> Vec<CodegenResult> {
    let mut results = vec![];

    for no in 0..ns.cfgs.len() {
        let cfg = &ns.cfgs[no];

        let filename = ns.files[0].clone();
        let context = Context::create();

        match target {
            "jit" => {
                let obj = ClassicTarget::build(&filename, cfg, &context, ns);
                let exit_code = obj.run_jit();
                results.push(CodegenResult::Jit { exit_code });
            }
            "wasm" => {
                lazy_static::initialize(&LLVM_INIT);
                let obj = WasmTarget::build(&filename, cfg, &context, ns);
                let code = obj.code().expect("compile should succeed");
                results.push(CodegenResult::Wasm { code });
            }
            "llvm" => {
                let obj = ClassicTarget::build(&filename, cfg, &context, ns);
                let name = format!("{}.ll", &cfg.name);
                match obj.dump_llvm(Path::new(&name)) {
                    Ok(_) => {
                        println!("dump llvm succeed: {:?}", name);
                    }
                    Err(_) => {
                        panic!("dump llvm failed: {:?}", name);
                    }
                }

                results.push(CodegenResult::LLVM {
                    value: "".to_string(),
                });
            }
            &_ => {
                let obj = ClassicTarget::build(&filename, cfg, &context, ns);
                let name = format!("{}.bc", &cfg.name);
                obj.bitcode(Path::new(&name));
                results.push(CodegenResult::BitCode);
            }
        }
    }

    results
}

#[cfg(test)]
mod test {
    #[test]
    #[rustfmt::skip]
    fn init_parser() {}
}
