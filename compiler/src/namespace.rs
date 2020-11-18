#[derive(Clone, Debug)]
// #[allow(clippy::large_enum_variant)]
pub enum Statement {
    VariableDecl()
}

#[derive(PartialEq, Clone, Debug)]
pub enum Expression {

}

#[derive(PartialEq, Clone, Debug)]
pub struct Parameter {
    pub name: String,
}

pub struct Function {
    pub name: String,
    pub params: Vec<Parameter>,
    pub returns: Vec<Parameter>,
    pub body: Vec<Statement>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Struct {
    pub name: String,
}

#[derive(PartialEq, Clone, Debug)]
pub struct StructDecl {
    pub name: String,
    pub functions: Vec<Function>,
}

pub struct Namespace {
    pub files: Vec<String>,
    pub structs: Vec<StructDecl>,
}

pub fn build(_filename: &str, _ns: &mut Namespace) {
    //
}
