use cjc_parser::{Program, ProgramUnit, StructFuncDecl};

use crate::neat::struct_function::struct_function_decl;
use crate::neat::{statements, Namespace};

pub fn resolve_program(program: Program, namespace: &mut Namespace) {
    // todo: make structs
    let _structs = program
        .0
        .iter()
        .filter_map(|part| {
            if let ProgramUnit::StructDecl(def) = part {
                Some(def)
            } else {
                None
            }
        })
        .enumerate()
        .map(|(no, def)| (no, def.as_ref()))
        .collect::<Vec<(usize, &cjc_parser::StructDecl)>>();

    // todo: resolve struct function
    let struct_funcs = program
        .0
        .iter()
        .filter_map(|part| {
            if let ProgramUnit::StructFuncDecl(def) = part {
                Some(def)
            } else {
                None
            }
        })
        .enumerate()
        .map(|(no, def)| (no, def.as_ref()))
        .collect::<Vec<(usize, &cjc_parser::StructFuncDecl)>>();

    // todo: add import support
    for part in &program.0 {
        match part {
            ProgramUnit::ImportDecl(_) => {}
            _ => {}
        }
    }

    resolve_struct_functions(struct_funcs, namespace);
}

pub fn resolve_struct_functions(
    struct_funcs: Vec<(usize, &StructFuncDecl)>,
    namespace: &mut Namespace,
) -> bool {
    let mut _broken = false;
    let mut function_bodies = Vec::new();

    for (index, func) in struct_funcs {
        struct_function_decl(func, namespace);
        if func.body.is_empty() {
        } else {
            function_bodies.push((index, func));
        }
    }

    for (index, def) in function_bodies {
        statements::resolve_function_body(def, namespace, index);
    }

    _broken
}
