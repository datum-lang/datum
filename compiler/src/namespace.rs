#[derive(PartialEq, Clone, Debug)]
pub struct StructDecl {
    pub name: String,
}

pub struct Namespace {
    pub files: Vec<String>,
    pub structs: Vec<StructDecl>,
}

pub fn build(_filename: &str, _ns: &mut Namespace) {
    //
}
