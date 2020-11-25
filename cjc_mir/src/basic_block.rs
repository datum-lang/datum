use crate::instruction::Instruction;

#[derive(Clone)]
pub struct BasicBlock {
    pub name: String,
    pub instructions: Vec<Instruction>,
}
