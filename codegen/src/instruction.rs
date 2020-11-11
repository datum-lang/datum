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
}
