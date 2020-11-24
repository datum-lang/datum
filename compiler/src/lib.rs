use inkwell::context::Context;

use cjc_parser::parser::parse_program;

use crate::neat::Namespace;

pub mod builtin;
pub mod symbol_table;
pub mod neat;
pub mod lowerify;

