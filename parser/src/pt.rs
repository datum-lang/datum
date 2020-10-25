use crate::location::Location;

#[derive(Debug, PartialEq)]
pub struct SourceUnit(pub Vec<SourceUnitPart>);

#[derive(Debug, PartialEq)]
pub enum SourceUnitPart {
    ImportDirective(Import),
}

#[derive(Debug, PartialEq)]
pub enum Import {
    Plain(StringLiteral),
    GlobalSymbol(StringLiteral, Identifier),
    Rename(StringLiteral, Vec<(Identifier, Option<Identifier>)>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct StringLiteral {
    pub loc: Location,
    pub string: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Identifier {
    pub loc: Location,
    pub name: String,
}

