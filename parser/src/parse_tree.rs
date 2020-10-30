use core::fmt;

use crate::location::{Loc, Location};
use num_bigint::BigInt;

#[derive(Debug, PartialEq)]
pub struct SourceUnit(pub Vec<SourceUnitPart>);

#[derive(Debug, PartialEq)]
pub enum SourceUnitPart {
    ImportDirective(Import),
    MultipleImportDirective(Vec<Import>),
    PackageDirective(Package),
    StructFuncDef(Box<StructFuncDef>),
    FuncDef(Box<FuncDef>),
    StructDef(Box<StructDef>),
}

pub type Suite = Vec<Statement>;

#[derive(Debug, PartialEq)]
pub struct FuncDef {
    pub loc: Loc,
    pub name: Identifier,
    pub params: Vec<(Loc, Option<Parameter>)>,
    pub body: Suite,
}

#[derive(Debug, PartialEq)]
pub struct StructFuncDef {
    pub loc: Loc,
    pub name: Identifier,
    pub struct_name: Identifier,
    pub params: Vec<(Loc, Option<Parameter>)>,
    pub body: Suite,
}

#[derive(Debug, PartialEq, Default)]
pub struct Parameters {
    pub args: Vec<Parameter>,
}

/// A single formal parameter to a function.
#[derive(Debug, PartialEq)]
pub struct Parameter {
    pub loc: Loc,
    pub ty: Expression,
    pub name: Option<Identifier>,
}

/// An expression at a given location in the sourcecode.
pub type Expression = Located<ExpressionType>;

/// A certain type of expression.
#[derive(Debug, PartialEq)]
pub enum ExpressionType {
    String {
        value: String,
    },

    /// A numeric literal.
    Number {
        value: Number,
    },

    /// A `list` literal value.
    List {
        elements: Vec<Expression>,
    },
    /// An identifier, designating a certain variable or type.
    Identifier {
        name: Identifier,
    },

    Type {
        ty: Type,
    },

    /// Attribute access in the form of `value.name`.
    Attribute {
        value: Box<Expression>,
        name: Identifier,
    },
    /// A call expression.
    Call {
        function: Box<Expression>,
        args: Vec<Argument>,
        // keywords: Vec<Keyword>,
    },

    /// A chained comparison. Note that in python you can use
    /// `1 < a < 10` for example.
    Compare {
        vals: Vec<Expression>,
        ops: Vec<Comparison>,
    },
}

#[derive(Debug, PartialEq)]
pub struct Argument {
    pub location: Location,
    pub expr: Expression,
}

#[derive(Debug, PartialEq)]
pub struct Keyword {
    pub name: Option<String>,
    pub value: Expression,
}

#[derive(Debug, PartialEq)]
pub struct Located<T> {
    pub location: Location,
    pub node: T,
}

pub type Statement = Located<StatementType>;

#[derive(Debug, PartialEq)]
pub enum StatementType {
    Break,
    Continue,
    If {
        cond: Expression,
        body: Suite,
        orelse: Option<Suite>,
    },
    While {
        cond: Expression,
        body: Suite,
    },
    For,
    Loop,
    /// Variable assignment. Note that we can assign to multiple targets.
    Assign {
        targets: Vec<Expression>,
        value: Expression,
    },
    Variable {
        field: Identifier,
        ty: Expression, // type
    },
    Return {
        value: Option<Expression>,
    },
    Expression {
        expression: Expression,
    },
}

#[derive(Debug, PartialEq)]
pub struct StructDef {
    pub loc: Loc,
    pub name: Identifier,
    pub fields: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub enum Package {
    Plain(Identifier),
}

#[derive(Debug, PartialEq)]
pub enum Import {
    Standard(Identifier),
    Remote,
    // for such github.com/phodal/coca
    GlobalSymbol(StringLiteral, Identifier),
    Rename(StringLiteral, Vec<(Identifier, Option<Identifier>)>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct StringLiteral {
    pub loc: Loc,
    pub string: String,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Identifier {
    pub loc: Loc,
    pub name: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DocComment {
    pub offset: usize,
    pub tag: String,
    pub value: String,
}

/// A comparison operation.
#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    NotEqual,
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
    In,
    NotIn,
    Is,
    IsNot,
}

/// A numeric literal.
#[derive(Debug, PartialEq)]
pub enum Number {
    Integer { value: BigInt },
    Float { value: f64 },
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Bool,
    String,
    Int(u16),
    Uint(u16),
    Bytes(u8),
    DynamicBytes,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Bool => write!(f, "bool"),
            Type::String => write!(f, "string"),
            Type::Int(n) => write!(f, "int{}", n),
            Type::Uint(n) => write!(f, "uint{}", n),
            Type::Bytes(n) => write!(f, "bytes{}", n),
            Type::DynamicBytes => write!(f, "bytes"),
        }
    }
}
