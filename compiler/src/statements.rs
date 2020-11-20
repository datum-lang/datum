use crate::expression::expression;
use crate::namespace::{Namespace, Statement};
use crate::symbol_table::SymbolTable;
use cjc_parser::StatementType;

pub fn statement(
    stmts: &Vec<cjc_parser::Statement>,
    res: &mut Vec<Statement>,
    ns: &mut Namespace,
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
