use crate::neat::expression::expression;
use crate::neat::Namespace;
use crate::symbol_table::SymbolTable;
use dc_hir::Statement;
use dc_parser::StructFuncDecl;

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
    body: &Vec<dc_parser::Statement>,
    res: &mut Vec<Statement>,
    namespace: &mut Namespace,
    symbol_table: &mut SymbolTable,
) {
    for stmt in body {
        match &stmt.node {
            dc_parser::StatementType::Break => {}
            dc_parser::StatementType::Continue => {}
            dc_parser::StatementType::If { .. } => {}
            dc_parser::StatementType::While { .. } => {}
            dc_parser::StatementType::For { .. } => {}
            dc_parser::StatementType::Loop => {}
            dc_parser::StatementType::Assign { .. } => {}
            dc_parser::StatementType::VariableDecl { .. } => {}
            dc_parser::StatementType::Return { .. } => {}
            dc_parser::StatementType::Expression { expr } => {
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
