use cjc_hir::Parameter;
use cjc_mir::basic_block::BasicBlock;
use cjc_mir::instruction::MIRKind;

#[derive(Clone)]
pub struct ControlFlowGraph {
    pub name: String,
    pub basic_block: BasicBlock,
    pub params: Vec<Parameter>,
    pub returns: Vec<Parameter>,
}

#[allow(dead_code)]
impl ControlFlowGraph {
    pub fn new(name: String) -> Self {
        ControlFlowGraph {
            name,
            basic_block: Default::default(),
            params: vec![],
            returns: vec![],
        }
    }

    pub fn placeholder() -> Self {
        ControlFlowGraph {
            name: "".to_string(),
            basic_block: Default::default(),
            params: vec![],
            returns: vec![],
        }
    }

    pub fn emit(&mut self, instruction: MIRKind) {
        self.basic_block.instructions.push(instruction);
    }
}
