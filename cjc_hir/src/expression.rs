use crate::Type;
use cjc_lexer::Location;

#[derive(Clone, Debug)]
pub enum Expression {
    Placeholder,
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
}
