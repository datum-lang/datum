use crate::{Parameter, Statement};

#[derive(Clone, Debug)]
pub struct Function {
    pub name: String,
    pub params: Vec<Parameter>,
    pub returns: Vec<Parameter>,
    pub body: Vec<Statement>,
}
