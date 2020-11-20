use crate::expression::expression;
use crate::symbol_table::SymbolTable;
use cjc_parser::StatementType;

pub fn statement(
    stmts: &Vec<cjc_parser::Statement>,
    res: &mut Vec<cjc_hir::Statement>,
    ns: &mut cjc_hir::Namespace,
    symbol_table: &mut SymbolTable,
) {
    for stmt in stmts {
        match &stmt.node {
            StatementType::Break => {}
            StatementType::Continue => {}
            StatementType::If { .. } => {}
            StatementType::While { .. } => {}
            StatementType::For { .. } => {}
            StatementType::Loop => {}
            StatementType::Assign { .. } => {}
            StatementType::Variable { .. } => {}
            StatementType::Return { .. } => {}
            StatementType::Expression { expr } => expression(&expr, ns, symbol_table),
        }
    }
}
