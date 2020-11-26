use cjc_lexer::Location;
use cjc_mir::basic_block::BasicBlock;
use cjc_mir::instruction::MIRKind;
use cjc_hir::Parameter;

#[derive(Clone)]
pub struct ControlFlowGraph {
    pub name: String,
    pub block: BasicBlock,
    pub params: Vec<Parameter>,
    pub returns: Vec<Parameter>,
}

#[allow(dead_code)]
impl ControlFlowGraph {
    pub fn new(name: String) -> Self {
        ControlFlowGraph {
            name,
            block: Default::default(),
            params: vec![],
            returns: vec![]
        }
    }

    pub fn placeholder() -> Self {
        ControlFlowGraph {
            name: "".to_string(),
            block: Default::default(),
            params: vec![],
            returns: vec![]
        }
    }

    fn emit(&mut self, instruction: MIRKind, _location: Location) {
        self.block.instructions.push(instruction);
    }
}
