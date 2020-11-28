use inkwell::context::Context;

use crate::lowerify::classic_target::ClassicTarget;
use crate::Namespace;
use std::path::Path;

pub mod classic_target;
pub mod code_object;
pub mod wasm_target;

pub fn codegen(ns: &mut Namespace) {
    for no in 0..ns.cfgs.len() {
        let cfg = &ns.cfgs[no];

        let filename = ns.files[0].clone();
        let context = Context::create();

        let obj = ClassicTarget::build(&filename, cfg, &context, ns);
        let name = format!("{}.bc", &cfg.name);
        obj.bitcode(Path::new(&name));
    }
}

#[cfg(test)]
mod test {
    #[test]
    #[rustfmt::skip]
    fn init_parser() {}
}
