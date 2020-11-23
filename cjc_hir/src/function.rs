use crate::{Parameter, Statement};

#[derive(Clone, Debug)]
pub struct Function {
    pub name: String,
    pub params: Vec<Parameter>,
    pub returns: Vec<Parameter>,
    pub body: Vec<Statement>,
}

impl Function {
    pub fn new(name: String, params: Vec<Parameter>, returns: Vec<Parameter>) -> Self {
        Function { name, params, returns, body: Vec::new() }
    }
}
