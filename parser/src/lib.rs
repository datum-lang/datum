#[macro_use]
extern crate lalrpop_util;

extern crate phf;

lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(unused)]
    pub charj
); // synthesized by LALRPOP

pub mod error;
pub mod lexer;
pub mod location;
pub mod parser;
pub mod ast;
pub mod token;
