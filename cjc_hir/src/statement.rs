use crate::Expression;
use cjc_lexer::Location;

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
