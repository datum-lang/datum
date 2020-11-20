use crate::symbol_table::SymbolTable;
use cjc_parser::{Argument, ExpressionType};
use cjc_hir::Namespace;

pub fn expression(
    expr: &cjc_parser::Expression,
    ns: &mut Namespace,
    symbol_table: &mut SymbolTable,
) {
    match &expr.node {
        ExpressionType::Range { .. } => {}
        ExpressionType::BoolOp { .. } => {}
        ExpressionType::Binop { .. } => {}
        ExpressionType::Unop { .. } => {}
        ExpressionType::String { .. } => {}
        ExpressionType::Bool { .. } => {}
        ExpressionType::Number { .. } => {}
        ExpressionType::List { .. } => {}
        ExpressionType::Identifier { .. } => {}
        ExpressionType::Type { .. } => {}
        ExpressionType::Attribute { .. } => {}
        ExpressionType::Call { function, args } => {
            function_call_expr(function, args, ns, symbol_table);
        }
        ExpressionType::SimpleCompare { .. } => {}
        ExpressionType::Compare { .. } => {}
    }
}

fn function_call_expr(
    function: &Box<cjc_parser::Expression>,
    args: &Vec<Argument>,
    ns: &mut Namespace,
    symtable: &SymbolTable,
) {
}
