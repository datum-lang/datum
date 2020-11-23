use cjc_hir::Namespace;
use cjc_parser::parser::parse_program;
use cjc_lexer::Diagnostic;
use cjc_parser::SourceUnit;
use crate::neat::unit::resolve_unit;

pub fn program(input: &str, filename: &str) {
    let mut namespace = Namespace::new();
    let parse_ast = parse_program(input);
    match parse_ast {
        Ok(unit) => {
            resolve_unit(unit, &mut namespace);
        }
        Err(_) => {}
    }
}

#[cfg(test)]
mod test {
    use crate::neat::program::program;

    #[test]
    fn init() {
        let result = program(
            "default$main() {println(\"hello,world\")}",
            "hello.cj",
        );
    }
}
