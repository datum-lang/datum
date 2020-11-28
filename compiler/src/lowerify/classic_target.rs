use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::targets::TargetTriple;

use crate::Namespace;

pub struct ClassicTarget {}

impl ClassicTarget {
    pub fn build<'a>(
        name: &String,
        context: &'a Context,
        ns: &'a mut Namespace,
    ) -> Self {
        Self {

        }
    }
}

