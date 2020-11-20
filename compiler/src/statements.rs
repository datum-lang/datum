use crate::namespace::Statement;
use cjc_parser::StatementType;

fn statement(
    stmt: &cjc_parser::Statement,
    res: &mut Vec<Statement>,
) {
    match stmt.node {
        StatementType::Break => {}
        StatementType::Continue => {}
        StatementType::If { .. } => {}
        StatementType::While { .. } => {}
        StatementType::For { .. } => {}
        StatementType::Loop => {}
        StatementType::Assign { .. } => {}
        StatementType::Variable { .. } => {}
        StatementType::Return { .. } => {}
        StatementType::Expression { .. } => {}
    }
}