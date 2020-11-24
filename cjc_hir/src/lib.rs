use cjc_lexer::Loc;
pub use expression::*;
pub use function::*;
pub use hir::*;
pub use statement::*;
pub use struct_def::*;
pub use types::*;

pub mod expression;
pub mod function;
pub mod hir;
pub mod statement;
pub mod struct_def;
pub mod types;

#[derive(Clone, Debug)]
pub struct Parameter {
    pub location: Loc,
    pub name: String,
}
