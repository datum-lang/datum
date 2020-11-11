use parser::parse_tree::Type;

#[derive(PartialEq, Clone, Debug)]
pub enum Builtin {
    Assert,
    Print,
}

pub struct Prototype {
    pub builtin: Builtin,
    pub namespace: Option<&'static str>,
    pub name: &'static str,
    pub args: &'static [Type],
    pub ret: &'static [Type],
    pub doc: &'static str,
}

// A list of all Solidity builtins functions
static BUILTIN_FUNCTIONS: [Prototype; 2] = [
    Prototype {
        builtin: Builtin::Print,
        namespace: Some("fmt"),
        name: "print",
        args: &[Type::String],
        ret: &[Type::Void],
        doc: "log string for debugging purposes. Runs on development chain only",
    },
    Prototype {
        builtin: Builtin::Assert,
        namespace: None,
        name: "assert",
        args: &[Type::Bool],
        ret: &[Type::Void],
        doc: "abort execution if argument evaluates to false",
    },
];
