use crate::Type;
use cjc_lexer::Location;

#[derive(PartialEq, Clone, Debug)]
pub enum Builtin {
    Assert,
    Print,
}

#[derive(Clone, Debug)]
pub enum Expression {
    Placeholder,
    // todo: thinking in change to bytes
    StringLiteral {
        location: Location,
        value: String,
    },
    BytesLiteral {
        location: Location,
        ty: Type,
        value: Vec<u8>,
    },
    InternalFunctionCall {
        location: Location,
        function: Box<Expression>,
        args: Vec<Expression>,
    },
    Builtin {
        location: Location,
        types: Vec<Type>,
        builtin: Builtin,
        args: Vec<Expression>,
    },
}
