use crate::instruction::ExprKind;

/// which is [basic block](https://en.wikipedia.org/wiki/Basic_block)
#[derive(Clone, Debug)]
pub struct BasicBlock {
    pub name: String,
    //  todo: ConditionKind ?
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
