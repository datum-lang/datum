use cjc_hir::StructDecl;

pub struct Namespace {
    // todo: add diagnostics
    pub files: Vec<String>,
    pub structs: Vec<StructDecl>,
}

impl Namespace {
    pub fn new() -> Self {
        Namespace {
            files: vec![],
            structs: vec![],
        }
    }

    pub fn resolve_type(&mut self, id: &cjc_parser::Expression) {

    }
}
