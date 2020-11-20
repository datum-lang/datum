pub use expression::*;
pub use function::*;
pub use hir::*;
pub use namespace::*;
pub use statement::*;
pub use struct_def::*;

pub mod expression;
pub mod function;
pub mod hir;
pub mod namespace;
pub mod statement;
pub mod struct_def;

#[derive(Clone, Debug)]
pub struct Parameter {
    pub name: String,
}
