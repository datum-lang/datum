use inkwell::context::Context;

use crate::lowerify::classic_target::ClassicTarget;
use crate::Namespace;
use std::path::Path;

pub mod classic_target;
pub mod struct_builder;
pub mod wasm_target;

pub fn codegen(ns: &mut Namespace) {
    for no in 0..ns.cfgs.len() {
        let cfg = &ns.cfgs[no];

        let filename = ns.files[0].clone();
        let context = Context::create();

        let compiler = ClassicTarget::build(&filename, cfg, &context, ns);
        compiler.bitcode(Path::new("main.cjc"));
    }
}

#[cfg(test)]
mod test {
    #[test]
    #[rustfmt::skip]
    fn init_parser() {}
}
