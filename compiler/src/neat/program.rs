use crate::neat::unit::resolve_unit;
use crate::neat::Namespace;
use cjc_parser::parser::parse_program;

pub fn program(input: &str, filename: &str) {
    let mut namespace = Namespace::new();
    namespace.files.push(filename.to_string());

    let parse_ast = parse_program(input);
    match parse_ast {
        Ok(unit) => {
            resolve_unit(unit, &mut namespace);
        }
        Err(_) => {}
    }
}
