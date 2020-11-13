use std::panic::Location;

use crate::instruction::Instruction;

#[derive(Clone)]
pub struct ControlFlowGraph {
    pub name: String,
    pub block: BasicBlock,
}

#[derive(Clone)]
pub struct BasicBlock {
    pub name: String,
    pub instructions: Vec<Instruction>,
}

impl ControlFlowGraph {
    fn emit(&mut self, instruction: Instruction, location: Location) {
        self.block.instructions.push(instruction);
    }
}
