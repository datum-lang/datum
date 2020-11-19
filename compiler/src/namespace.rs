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

#[derive(Clone, Debug)]
pub enum Expression {
    InternalFunctionCall {
        location: Location,
        function: Box<Expression>,
        args: Vec<Expression>,
    },
}

#[derive(Clone, Debug)]
pub struct Parameter {
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct Function {
    pub name: String,
    pub params: Vec<Parameter>,
    pub returns: Vec<Parameter>,
    pub body: Vec<Statement>,
}

#[derive(Clone, Debug)]
pub struct Struct {
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct StructDecl {
    pub name: String,
    pub functions: Vec<Function>,
}

pub struct Namespace {
    // todo: add diagnostics
    pub files: Vec<String>,
    pub structs: Vec<StructDecl>,
}

impl Namespace {
    pub fn new() -> Self {
        Namespace {
            files: vec![],
            structs: vec![]
        }
    }
}

pub fn build(_filename: &str, _ns: &mut Namespace) {
    //
}
