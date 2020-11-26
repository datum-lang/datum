use crate::{ControlFlowGraph, Namespace};
use cjc_hir::{Builtin, Expression, Function, Statement};
use cjc_mir::instruction::MIRKind;

pub fn meanify(ns: &mut Namespace) {
    let mut cfg_no = 0;
    let mut all_cfg = Vec::new();

    cfg_no = ns.functions.len();
    all_cfg.resize(cfg_no, ControlFlowGraph::placeholder());

    let function_no = 0;
    for _cfg in all_cfg {
        function_cfg(function_no, ns);
    }
}

pub fn function_cfg(function_no: usize, ns: &mut Namespace) {
    let mut func = &ns.functions[function_no];

    let func_name = &func.name;
    let mut cfg = ControlFlowGraph::new(func_name.to_string());
    cfg.params = func.params.clone();
    cfg.returns = func.returns.clone();

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
        Statement::VariableDecl { location } => {
            // todo
        }
        Statement::Expression {
            location,
            expression: expr,
        } => {
            expression_cfg(expr, cfg, ns);
        }
    }
}

pub fn expression_cfg(expr: &Expression, cfg: &mut ControlFlowGraph, ns: &Namespace) -> Expression {
    match expr {
        Expression::Placeholder => Expression::Placeholder,
        Expression::StringLiteral { .. } => Expression::Placeholder,
        Expression::BytesLiteral { .. } => Expression::Placeholder,
        Expression::InternalFunctionCall { .. } => Expression::Placeholder,
        Expression::Builtin {
            location,
            types,
            builtin,
            args,
        } => match builtin {
            Builtin::Assert => Expression::Placeholder,
            Builtin::Print => {
                println!("{:?}", &args[0]);
                let expr = expression_cfg(&args[0], cfg, ns);
                // cfg.emit(MIRKind::Call {});
                expr
            }
        },
    }
}
