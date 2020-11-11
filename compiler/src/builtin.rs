use parser::location::Location;
use parser::parse_tree::Type;
use std::collections::HashMap;

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

#[derive(Clone, PartialEq)]
pub enum Symbol {
    Function(Vec<Location>),
    Variable(Location, usize, usize),
    Struct(Location, usize),
    Import(Location, usize),
}

/// Does function call match builtin
pub fn is_builtin_call(namespace: Option<&str>, fname: &str) -> bool {
    BUILTIN_FUNCTIONS
        .iter()
        .any(|p| p.name == fname && p.namespace == namespace)
}

#[cfg(test)]
mod tests {
    use crate::builtin::is_builtin_call;

    #[test]
    fn should_identify_builtin_print() {
        let is_builtin = is_builtin_call(Some("fmt"), "print");
        assert_eq!(true, is_builtin);

        let no_builtin = is_builtin_call(Some("fmt"), "printf");
        assert_eq!(false, no_builtin);
    }
}
