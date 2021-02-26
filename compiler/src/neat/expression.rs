use dc_hir::{Expression, Type};
use dc_parser::{Argument, ExpressionType};

use crate::builtin;
use crate::neat::Namespace;
use crate::symbol_table::SymbolTable;

pub fn expression(
    expr: &dc_parser::Expression,
    ns: &mut Namespace,
    symbol_table: &mut SymbolTable,
) -> Result<dc_hir::Expression, ()> {
    match &expr.node {
        ExpressionType::Range { .. } => Ok(dc_hir::Expression::Placeholder),
        ExpressionType::BoolOp { .. } => Ok(dc_hir::Expression::Placeholder),
        ExpressionType::Binop { .. } => Ok(dc_hir::Expression::Placeholder),
        ExpressionType::Unop { .. } => Ok(dc_hir::Expression::Placeholder),
        ExpressionType::String { value } => Ok(dc_hir::Expression::StringLiteral {
            location: *&expr.location,
            value: value.to_string(),
        }),
        ExpressionType::Bool { .. } => Ok(dc_hir::Expression::Placeholder),
        ExpressionType::Number { value } => {
            let bits = value.bits();
            let int_size = if bits < 7 { 8 } else { (bits + 7) & !7 } as u16;

            Ok(dc_hir::Expression::NumberLiteral {
                location: *&expr.location,
                ty: Type::Int(int_size),
                value: value.clone(),
            })
        }
        ExpressionType::List { .. } => Ok(dc_hir::Expression::Placeholder),
        ExpressionType::Identifier { id: name } => Ok(dc_hir::Expression::Variable {
            location: *&expr.location,
            ty: Type::String,
            value: name.name.clone(),
        }),
        ExpressionType::Type { .. } => Ok(dc_hir::Expression::Placeholder),
        ExpressionType::MemberAccess { .. } => Ok(dc_hir::Expression::Placeholder),
        ExpressionType::Call { function, args } => {
            let result = function_call_expr(function, args, ns, symbol_table);
            return result;
        }
        ExpressionType::Compare { .. } => Ok(dc_hir::Expression::Placeholder),
        ExpressionType::PostUnop { .. } => Ok(dc_hir::Expression::Placeholder),
        ExpressionType::EmptyObject => Ok(dc_hir::Expression::Placeholder),
    }
}

fn function_call_expr(
    function: &Box<dc_parser::Expression>,
    args: &Vec<Argument>,
    ns: &mut Namespace,
    symtable: &mut SymbolTable,
) -> Result<Expression, ()> {
    // todo: match for MemberAccess
    function_call(function, args, ns, symtable)
}

fn function_call(
    var: &Box<dc_parser::Expression>,
    args: &Vec<Argument>,
    ns: &mut Namespace,
    symbol_table: &mut SymbolTable,
) -> Result<Expression, ()> {
    match &var.node {
        ExpressionType::Identifier { id } => {
            let is_builtin = builtin::is_builtin_call(None, &*id.name);

            if is_builtin {
                let result =
                    builtin::resolve_call(&var.location, None, ns, &*id.name, args, symbol_table);
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
