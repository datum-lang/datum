use crate::instruction::MIRKind;

#[derive(Clone)]
pub struct BasicBlock {
    pub name: String,
    pub instructions: Vec<MIRKind>,
}
