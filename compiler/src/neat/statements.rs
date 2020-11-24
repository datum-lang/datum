use crate::neat::expression::expression;
use crate::neat::Namespace;
use crate::symbol_table::SymbolTable;
use cjc_parser::StructFuncDef;

pub fn resolve_function_body(func_def: &StructFuncDef, namespace: &mut Namespace) {
    let mut symbol_table = SymbolTable::new();
    statement(&func_def.body, namespace, &mut symbol_table)
}

pub fn statement(
    body: &Vec<cjc_parser::Statement>,
    namespace: &mut Namespace,
    symbol_table: &mut SymbolTable,
) {
    for stmt in body {
        match &stmt.node {
            cjc_parser::StatementType::Break => {}
            cjc_parser::StatementType::Continue => {}
            cjc_parser::StatementType::If { .. } => {}
            cjc_parser::StatementType::While { .. } => {}
            cjc_parser::StatementType::For { .. } => {}
            cjc_parser::StatementType::Loop => {}
            cjc_parser::StatementType::Assign { .. } => {}
            cjc_parser::StatementType::Variable { .. } => {}
            cjc_parser::StatementType::Return { .. } => {}
            cjc_parser::StatementType::Expression { expr } => {
                let _r = expression(&expr, namespace, symbol_table);
            }
        }
    }
}
