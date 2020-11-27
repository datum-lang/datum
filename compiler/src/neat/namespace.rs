use crate::ControlFlowGraph;
use cjc_hir::{Function, StructDecl};
use cjc_parser::ExpressionType;

#[derive(Debug)]
pub struct Namespace {
    // todo: add diagnostics
    pub files: Vec<String>,
    pub structs: Vec<StructDecl>,
    pub functions: Vec<Function>,
    pub cfgs: Vec<ControlFlowGraph>,
}

impl Namespace {
    pub fn new() -> Self {
        Namespace {
            files: vec![],
            structs: vec![],
            functions: vec![],
            cfgs: vec![],
        }
    }

    pub fn resolve_type(&mut self, id: &cjc_parser::Expression) {
        self.expr_to_type(&id);
    }

    pub fn expr_to_type<'a>(&mut self, expr: &'a cjc_parser::Expression) {
        let expr = expr;
        match expr.node {
            ExpressionType::Call { .. } => {}
            _ => {
                println!("{:?}", expr.node);
            }
        }
    }
}
