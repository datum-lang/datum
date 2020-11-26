use cjc_lexer::Location;
use cjc_mir::basic_block::BasicBlock;
use cjc_mir::instruction::MIRKind;

#[derive(Clone)]
pub struct ControlFlowGraph {
    pub name: String,
    pub block: BasicBlock,
}

#[allow(dead_code)]
impl ControlFlowGraph {
    pub fn new(name: String) -> Self {
        ControlFlowGraph {
            name,
            block: Default::default(),
        }
    }

    pub fn placeholder() -> Self {
        ControlFlowGraph {
            name: "".to_string(),
            block: Default::default(),
        }
    }

    fn emit(&mut self, instruction: MIRKind, _location: Location) {
        self.block.instructions.push(instruction);
    }
}
