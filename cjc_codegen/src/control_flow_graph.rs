use std::panic::Location;

use crate::instruction::Instruction;

#[derive(Clone)]
pub struct BasicBlock {
    pub name: String,
    pub instructions: Vec<Instruction>,
}

#[derive(Clone)]
pub struct ControlFlowGraph {
    pub name: String,
    pub block: BasicBlock,
}

#[allow(dead_code)]
impl ControlFlowGraph {
    pub fn placeholder(name: String, block: BasicBlock) -> Self {
        ControlFlowGraph { name, block }
    }

    fn emit(&mut self, instruction: Instruction, _location: Location) {
        self.block.instructions.push(instruction);
    }
}
