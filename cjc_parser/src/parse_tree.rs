use core::fmt;

use num_bigint::BigInt;

use cjc_lexer::{Loc, Location};

#[derive(Debug, PartialEq)]
pub struct Program(pub Vec<ProgramUnit>);

#[derive(Debug, PartialEq)]
pub enum ProgramUnit {
    PackageDecl(Package),
    ImportDecl(Import),
    StructFuncDecl(Box<StructFuncDecl>),
    FuncDecl(Box<FuncDecl>),
    StructDecl(Box<StructDecl>),
    ObjectDecl(Box<ObjectDecl>),
}

#[derive(Debug, PartialEq)]
pub struct ObjectDecl {
    pub loc: Loc,
    pub name: Identifier,
    pub functions: Vec<Box<FuncDecl>>,
}

#[derive(Debug, PartialEq)]
pub struct FuncDecl {
    pub loc: Loc,
    pub name: Identifier,
    pub params: Vec<(Loc, Option<Parameter>)>,
    pub body: Option<Statement>,
}

#[derive(Debug, PartialEq)]
pub struct StructFuncDecl {
    pub loc: Loc,
    pub name: Identifier,
    pub struct_name: Identifier,
    pub params: Vec<(Loc, Option<Parameter>)>,
    pub body: Option<Statement>,
    pub returns: Option<Expression>,
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

impl Parameter {
    pub fn get_name(&self) -> String {
        self.name.as_ref().unwrap().clone().name
    }
}

/// An expression at a given location in the sourcecode.
pub type Expression = Located<ExpressionType>;

/// A certain type of expression.
#[derive(Debug, PartialEq)]
pub enum ExpressionType {
    Range {
        start: Box<Expression>,
        end: Box<Expression>,
    },
    BoolOp {
        op: BooleanOperator,
        values: Vec<Expression>,
    },

    /// A binary operation on two operands.
    Binop {
        a: Box<Expression>,
        op: Operator,
        b: Box<Expression>,
    },

    /// An unary operation.
    Unop {
        op: UnaryOperator,
        a: Box<Expression>,
    },

    String {
        value: String,
    },

    Bool {
        value: bool,
    },

    /// A numeric literal.
    Number {
        value: BigInt,
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
    MemberAccess {
        value: Box<Expression>,
        name: Identifier,
    },
    /// A call expression.
    Call {
        function: Box<Expression>,
        args: Vec<Argument>,
        // keywords: Vec<Keyword>,
    },
    Compare {
        op: Comparison,
        left: Box<Expression>,
        right: Box<Expression>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum VariableStorage {
    Memory { location: Location },
    Storage { location: Location },
}

impl VariableStorage {
    pub fn location(&self) -> &Location {
        match self {
            VariableStorage::Memory { location } => location,
            VariableStorage::Storage { location } => location,
        }
    }
}

impl fmt::Display for VariableStorage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VariableStorage::Memory { .. } => write!(f, "memory"),
            VariableStorage::Storage { .. } => write!(f, "storage"),
        }
    }
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
#[allow(clippy::large_enum_variant, clippy::type_complexity)]
pub enum StatementType {
    VariableDecl {
        field: Identifier,
        ty: Expression, // type
    },
    Break,
    Block {
        blocks: Vec<Statement>
    },
    Continue,
    If {
        cond: Expression,
        body: Box<Statement>,
        orelse: Option<Box<Statement>>,
    },
    While {
        cond: Expression,
        body: Box<Statement>,
    },
    For {
        target: Box<Expression>,
        iter: Box<Expression>,
        body: Option<Box<Statement>>,
    },
    Loop,
    /// Variable assignment. Note that we can assign to multiple targets.
    Assign {
        target: Identifier,
        value: Expression,
        ty: Expression,
    },
    Return {
        value: Option<Expression>,
    },
    Expression {
        expr: Expression,
    },
}

#[derive(Debug, PartialEq)]
pub struct StructDecl {
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

/// An operator for a binary operation (an operation with two operands).
#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Multiply,
    MatMult,
    Divide,
    Modulo,
    Pow,
    LShift,
    RShift,
    BitOr,
    BitXor,
    BitAnd,
    FloorDiv, // from RustPython, thinking in remove
}

/// An unary operator. This is an operation with only a single operand.
#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    Pos,
    Neg,
    Not,
    Inv,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Bool,
    String,
    Int(u16),
    Uint(u16),
    Bytes(u8),
    DynamicBytes,
    Void,
}

/// A boolean operation.
#[derive(Debug, PartialEq)]
pub enum BooleanOperator {
    And,
    Or,
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
            Type::Void => write!(f, "void"),
        }
    }
}
