use cjc_parser::{SourceUnit, SourceUnitPart, StructFuncDef};

use crate::neat::struct_function::struct_function_decl;
use crate::neat::{statements, Namespace};

pub fn resolve_unit(unit: SourceUnit, namespace: &mut Namespace) {
    let _structs = unit
        .0
        .iter()
        .filter_map(|part| {
            if let SourceUnitPart::StructDef(def) = part {
                Some(def)
            } else {
                None
            }
        })
        .enumerate()
        .map(|(no, def)| (no, def.as_ref()))
        .collect::<Vec<(usize, &cjc_parser::StructDef)>>();

    // todo: resolve struct function
    let struct_funcs = unit
        .0
        .iter()
        .filter_map(|part| {
            if let SourceUnitPart::StructFuncDef(def) = part {
                Some(def)
            } else {
                None
            }
        })
        .enumerate()
        .map(|(no, def)| (no, def.as_ref()))
        .collect::<Vec<(usize, &cjc_parser::StructFuncDef)>>();

    // todo: add import support
    for part in &unit.0 {
        match part {
            SourceUnitPart::ImportDirective(_) => {}
            _ => {}
        }
    }

    resolve_struct_functions(struct_funcs, namespace);
}

pub fn resolve_struct_functions(
    struct_funcs: Vec<(usize, &StructFuncDef)>,
    namespace: &mut Namespace,
) -> bool {
    let mut _broken = false;

    // let mut function_no_bodies = Vec::new();
    let mut function_bodies = Vec::new();

    for (_index, func) in struct_funcs {
        struct_function_decl(func, namespace);
        if func.body.is_empty() {
        } else {
            function_bodies.push(func);
        }
    }

    resolve_bodies(function_bodies, namespace);

    _broken
}

pub fn resolve_bodies(bodies: Vec<&StructFuncDef>, namespace: &mut Namespace) {
    for def in bodies {
        statements::resolve_function_body(def, namespace);
    }
}
