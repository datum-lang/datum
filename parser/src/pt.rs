use crate::location::Loc;

#[derive(Debug, PartialEq)]
pub struct SourceUnit(pub Vec<SourceUnitPart>);

#[derive(Debug, PartialEq)]
pub enum SourceUnitPart {
    ImportDirective(Import),
    MultipleImportDirective(Vec<Import>),
    PackageDirective(Package),
    DefaultFunctionDefinition(Box<DefaultFunctionDefinition>),
    FunctionDefinition(Box<FunctionDefinition>),
    StructDefinition(Box<StructDefinition>),
}

#[derive(Debug, PartialEq)]
pub struct DefaultFunctionDefinition {
    pub loc: Loc,
    pub name: Identifier,
}

#[derive(Debug, PartialEq)]
pub struct FunctionDefinition {
    pub loc: Loc,
    pub name: Identifier,
}

#[derive(Debug, PartialEq)]
pub struct StructDefinition {
    pub loc: Loc,
    pub name: Identifier,
    pub fields: Vec<VariableDeclaration>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableDeclaration {}

#[derive(Debug, PartialEq)]
pub enum Package {
    Plain(Identifier),
}

#[derive(Debug, PartialEq)]
pub enum Import {
    Standard(Identifier),
    Remote, // for such github.com/phodal/coca
    GlobalSymbol(StringLiteral, Identifier),
    Rename(StringLiteral, Vec<(Identifier, Option<Identifier>)>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct StringLiteral {
    pub loc: Loc,
    pub string: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Identifier {
    pub loc: Loc,
    pub name: String,
}
