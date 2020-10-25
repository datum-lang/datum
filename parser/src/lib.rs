#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(unused)]
    pub charj
); // synthesized by LALRPOP

pub mod ast;
pub mod error;
pub mod lexer;
pub mod location;
pub mod parser;
pub mod token;
