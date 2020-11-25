use serde::{Deserialize, Serialize};
use num_bigint::BigInt;


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Constant {
    Integer { value: BigInt },
    Float { value: f64 },
    Boolean { value: bool },
    String { value: String },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Instruction {
    ImportFrom {
        name: String,
    },
    LoadName {
        name: String,
    },
    LoadConst {
        value: Constant,
    },
    CallFunction {
        // expr: Expression,
    }
}