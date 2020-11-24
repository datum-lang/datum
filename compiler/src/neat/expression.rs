use crate::builtin;
use crate::neat::Namespace;
use crate::symbol_table::SymbolTable;
use cjc_hir::Expression;
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
        ExpressionType::String { value } => Ok(cjc_hir::Expression::StringLiteral {
            location: *&expr.location,
            value: value.to_string(),
        }),
        ExpressionType::Bool { .. } => Ok(cjc_hir::Expression::Placeholder),
        ExpressionType::Number { .. } => Ok(cjc_hir::Expression::Placeholder),
        ExpressionType::List { .. } => Ok(cjc_hir::Expression::Placeholder),
        ExpressionType::Identifier { .. } => Ok(cjc_hir::Expression::Placeholder),
        ExpressionType::Type { .. } => Ok(cjc_hir::Expression::Placeholder),
        ExpressionType::Attribute { .. } => Ok(cjc_hir::Expression::Placeholder),
        ExpressionType::Call { function, args } => {
            let result = function_call_expr(function, args, ns, symbol_table);
            return result;
        }
        ExpressionType::SimpleCompare { .. } => Ok(cjc_hir::Expression::Placeholder),
        ExpressionType::Compare { .. } => Ok(cjc_hir::Expression::Placeholder),
    }
}

fn function_call_expr(
    function: &Box<cjc_parser::Expression>,
    args: &Vec<Argument>,
    ns: &mut Namespace,
    symtable: &mut SymbolTable,
) -> Result<Expression, ()> {
    method_call(function, args, ns, symtable)
}

fn method_call(
    var: &Box<cjc_parser::Expression>,
    args: &Vec<Argument>,
    ns: &mut Namespace,
    symbol_table: &mut SymbolTable,
) -> Result<Expression, ()> {
    match &var.node {
        ExpressionType::Identifier { name } => {
            let is_builtin = builtin::is_builtin_call(None, &*name.name);
            if is_builtin {
                let result =
                    builtin::resolve_call(&*name.name, None, ns, &*name.name, args, symbol_table);
                return result;
            }
        }
        _ => {}
    }

    return Err(());
}
