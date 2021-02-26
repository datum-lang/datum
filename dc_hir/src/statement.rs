use crate::Expression;
use dc_lexer::Location;

#[derive(Clone, Debug)]
pub enum Statement {
    VariableDecl {
        location: Location,
    },
    Expression {
        location: Location,
        expression: Expression,
    },
}
