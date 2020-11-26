use crate::{ControlFlowGraph, Namespace};
use cjc_hir::{Function, Statement};

pub fn meanify(ns: &mut Namespace) {
    let mut cfg_no = 0;
    let mut all_cfg = Vec::new();

    cfg_no = ns.functions.len();
    all_cfg.resize(cfg_no, ControlFlowGraph::placeholder());

    for function in ns.functions.iter_mut() {
        // function_cfg(function, ns);
    }
}

pub fn function_cfg(func: &mut Function, ns: &mut Namespace) {
    let func_name = &func.name;
    let mut cfg = ControlFlowGraph::new(func_name.to_string());
    // cfg.params = func.params.clone();
    // cfg.returns = func.returns.clone();

    for stmt in &func.body {
        statement_cfg(stmt, func, &mut cfg, ns)
    }
}

pub fn statement_cfg(
    stmt: &Statement,
    func: &Function,
    cfg: &mut ControlFlowGraph,
    ns: &Namespace,
) {
    match stmt {
        Statement::VariableDecl { .. } => {}
        Statement::Expression { .. } => {}
    }
}
