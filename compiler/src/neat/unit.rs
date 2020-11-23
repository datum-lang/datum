use cjc_parser::{SourceUnit, SourceUnitPart};
use cjc_hir::Namespace;

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
}