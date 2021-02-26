use crate::ControlFlowGraph;
use dc_hir::{Function, StructDecl};
use dc_parser::ExpressionType;

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

    pub fn resolve_type(&mut self, id: &dc_parser::Expression) {
        self.expr_to_type(&id);
    }

    pub fn expr_to_type<'a>(&mut self, expr: &'a dc_parser::Expression) {
        let expr = expr;
        match expr.node {
            ExpressionType::Call { .. } => {}
            _ => {
                println!("{:?}", expr.node);
            }
        }
    }
}
