use dc_hir::Parameter;
use dc_mir::basic_block::BasicBlock;
use dc_mir::instruction::ExprKind;

///  which is a [Control-flow graph](https://en.wikipedia.org/wiki/Control-flow_graph)
#[derive(Clone, Debug)]
pub struct ControlFlowGraph {
    pub name: String,
    pub blocks: BasicBlock,
    pub params: Vec<Parameter>,
    pub returns: Vec<Parameter>,
}

#[allow(dead_code)]
impl ControlFlowGraph {
    pub fn new(name: String) -> Self {
        ControlFlowGraph {
            name,
            blocks: Default::default(),
            params: vec![],
            returns: vec![],
        }
    }

    pub fn placeholder() -> Self {
        ControlFlowGraph {
            name: "".to_string(),
            blocks: Default::default(),
            params: vec![],
            returns: vec![],
        }
    }

    pub fn emit(&mut self, instruction: ExprKind) {
        self.blocks.instructions.push(instruction);
    }
}
