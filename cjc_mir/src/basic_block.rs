use crate::instruction::MIRKind;

#[derive(Clone)]
pub struct BasicBlock {
    pub name: String,
    pub instructions: Vec<MIRKind>,
}

impl Default for BasicBlock {
    fn default() -> Self {
        BasicBlock {
            name: "".to_string(),
            instructions: vec![],
        }
    }
}

impl BasicBlock {}
