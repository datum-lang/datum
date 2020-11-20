use cjc_lexer::Location;

#[derive(Clone, Debug)]
pub enum Expression {
    InternalFunctionCall {
        location: Location,
        function: Box<Expression>,
        args: Vec<Expression>,
    },
}
