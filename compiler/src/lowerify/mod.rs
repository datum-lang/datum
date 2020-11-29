use inkwell::context::Context;

use crate::lowerify::classic_target::ClassicTarget;
use crate::Namespace;
use std::path::Path;

pub mod classic_target;
pub mod code_object;
pub mod wasm_target;
pub mod base_target;

pub fn codegen(ns: &mut Namespace, target: &str) {
    for no in 0..ns.cfgs.len() {
        let cfg = &ns.cfgs[no];

        let filename = ns.files[0].clone();
        let context = Context::create();

        let obj = ClassicTarget::build(&filename, cfg, &context, ns);
        match target {
            "jit" => {
                obj.run_jit();
            }
            "llvm" => {
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
