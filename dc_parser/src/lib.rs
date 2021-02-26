#[macro_use]
extern crate lalrpop_util;
extern crate phf;

lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(unused)]
    pub datum
); // synthesized by LALRPOP

pub mod parse_tree;
pub mod parser;

pub use parse_tree::*;
