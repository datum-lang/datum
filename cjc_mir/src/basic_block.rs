use crate::instruction::ExprKind;

#[derive(Clone, Debug)]
pub struct BasicBlock {
    pub name: String,
    pub instructions: Vec<ExprKind>,
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
