use crate::Function;

#[derive(Clone, Debug)]
pub struct Struct {
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct StructDecl {
    pub name: String,
    pub functions: Vec<Function>,
}
