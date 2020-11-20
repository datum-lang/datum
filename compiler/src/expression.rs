use crate::symbol_table::SymbolTable;
use cjc_hir::Namespace;
use cjc_parser::{Argument, ExpressionType};

pub fn expression(
    expr: &cjc_parser::Expression,
    ns: &mut Namespace,
    symbol_table: &mut SymbolTable,
) -> Result<cjc_hir::Expression, ()> {
    match &expr.node {
        ExpressionType::Range { .. } => Ok(cjc_hir::Expression::Placeholder),
        ExpressionType::BoolOp { .. } => Ok(cjc_hir::Expression::Placeholder),
        ExpressionType::Binop { .. } => Ok(cjc_hir::Expression::Placeholder),
        ExpressionType::Unop { .. } => Ok(cjc_hir::Expression::Placeholder),
        ExpressionType::String { .. } => Ok(cjc_hir::Expression::Placeholder),
        ExpressionType::Bool { .. } => Ok(cjc_hir::Expression::Placeholder),
        ExpressionType::Number { .. } => Ok(cjc_hir::Expression::Placeholder),
        ExpressionType::List { .. } => Ok(cjc_hir::Expression::Placeholder),
        ExpressionType::Identifier { .. } => Ok(cjc_hir::Expression::Placeholder),
        ExpressionType::Type { .. } => Ok(cjc_hir::Expression::Placeholder),
        ExpressionType::Attribute { .. } => Ok(cjc_hir::Expression::Placeholder),
        ExpressionType::Call { function, args } => {
            function_call_expr(function, args, ns, symbol_table);
            Ok(cjc_hir::Expression::Placeholder)
        }
        ExpressionType::SimpleCompare { .. } => Ok(cjc_hir::Expression::Placeholder),
        ExpressionType::Compare { .. } => Ok(cjc_hir::Expression::Placeholder),
    }
}

fn function_call_expr(
    function: &Box<cjc_parser::Expression>,
    args: &Vec<Argument>,
    ns: &mut Namespace,
    symtable: &SymbolTable,
) {
}
