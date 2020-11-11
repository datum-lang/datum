use num_bigint::BigInt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Constant {
    Integer { value: BigInt },
    Float { value: f64 },
    Boolean { value: bool },
    String { value: String },
    Bytes { value: Vec<u8> },
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
/// An indication where the name must be accessed.
pub enum NameScope {
    /// The name will be in the local scope.
    Local,

    /// The name will be located in scope surrounding the current scope.
    NonLocal,

    /// The name will be in global scope.
    Global,

    /// The name will be located in any scope between the current scope and the top scope.
    Free,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Instruction {
    ImportFrom {
        name: String,
    },
    Print {
        // expr: Expression,
    },
    Constructor {},
    LoadConst {
        value: Constant,
    },
    LoadName {
        name: String,
        scope: NameScope,
    },
}
