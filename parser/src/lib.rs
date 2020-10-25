#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(unused)]
    pub charj
); // synthesized by LALRPOP

pub mod error;
pub mod lexer;
pub mod location;
pub mod parser;
pub mod pt;
pub mod token;
