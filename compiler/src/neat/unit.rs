use cjc_parser::{SourceUnit, SourceUnitPart, StructFuncDef};
use crate::neat::struct_function::struct_function;
use crate::neat::Namespace;

pub fn resolve_unit(unit: SourceUnit, namespace: &mut Namespace) {
    let _structs = unit.0.iter()
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
    let struct_funcs = unit.0.iter()
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

pub fn resolve_struct_functions(struct_funcs: Vec<(usize, &StructFuncDef)>, namespace: &mut Namespace) -> bool {
    let mut _broken = false;
    for (_index, func) in struct_funcs {
        let _result = struct_function(func, namespace);
    }

    _broken
}