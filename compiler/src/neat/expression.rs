use cjc_hir::{Expression, Type};
use cjc_parser::{Argument, ExpressionType};

use crate::builtin;
use crate::neat::Namespace;
use crate::symbol_table::SymbolTable;

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
        ExpressionType::Number { value } => {
            let bits = value.bits();
            let int_size = if bits < 7 { 8 } else { (bits + 7) & !7 } as u16;

            Ok(cjc_hir::Expression::NumberLiteral {
                location: *&expr.location,
                ty: Type::Int(int_size),
                value: value.clone(),
            })
        }
        ExpressionType::List { .. } => Ok(cjc_hir::Expression::Placeholder),
        ExpressionType::Identifier { name } => Ok(cjc_hir::Expression::Variable {
            location: *&expr.location,
            ty: Type::String,
            value: name.name.clone(),
        }),
        ExpressionType::Type { .. } => Ok(cjc_hir::Expression::Placeholder),
        ExpressionType::MemberAccess { .. } => Ok(cjc_hir::Expression::Placeholder),
        ExpressionType::Call { function, args } => {
            let result = function_call_expr(function, args, ns, symbol_table);
            return result;
        }
        ExpressionType::Compare { .. } => Ok(cjc_hir::Expression::Placeholder),
        ExpressionType::PostUnop { .. } => Ok(cjc_hir::Expression::Placeholder),
        ExpressionType::EmptyObject => Ok(cjc_hir::Expression::Placeholder),
    }
}

fn function_call_expr(
    function: &Box<cjc_parser::Expression>,
    args: &Vec<Argument>,
    ns: &mut Namespace,
    symtable: &mut SymbolTable,
) -> Result<Expression, ()> {
    // todo: match for MemberAccess
    function_call(function, args, ns, symtable)
}

fn function_call(
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
                    builtin::resolve_call(&var.location, None, ns, &*name.name, args, symbol_table);
                return result;
            }

            let function = expression(var, ns, symbol_table)?;
            return Ok(Expression::InternalFunctionCall {
                location: var.location,
                args: vec![],
                function: Box::new(function),
            });
        }
        _ => {
            println!("{:?}", &var.node);
        }
    }

    return Err(());
}
