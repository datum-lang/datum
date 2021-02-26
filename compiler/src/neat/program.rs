use crate::neat::unit::resolve_program;
use crate::neat::Namespace;
use dc_parser::parser::parse_program;

pub fn program(input: &str, _filename: &str, namespace: &mut Namespace) {
    let parse_ast = parse_program(input);
    match parse_ast {
        Ok(unit) => {
            resolve_program(unit, namespace);
        }
        Err(_) => {}
    }
}
