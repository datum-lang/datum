use crate::neat::expression::expression;
use crate::neat::Namespace;
use crate::symbol_table::SymbolTable;
use cjc_hir::Statement;
use cjc_parser::StructFuncDecl;

pub fn resolve_function_body(
    func_def: &StructFuncDecl,
    namespace: &mut Namespace,
    function_no: usize,
) {
    let mut res = Vec::new();
    let mut symbol_table = SymbolTable::new();

    statement(&func_def.body, &mut res, namespace, &mut symbol_table);

    namespace.functions[function_no].body = res;
}

pub fn statement(
    body: &Vec<cjc_parser::Statement>,
    res: &mut Vec<Statement>,
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
            cjc_parser::StatementType::VariableDecl { .. } => {}
            cjc_parser::StatementType::Return { .. } => {}
            cjc_parser::StatementType::Expression { expr } => {
                let result = expression(&expr, namespace, symbol_table);
                match result {
                    Ok(expression) => {
                        res.push(Statement::Expression {
                            location: stmt.location,
                            expression,
                        });
                    }
                    Err(_) => {}
                }
            }
        }
    }
}
