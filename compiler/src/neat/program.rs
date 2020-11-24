use cjc_parser::parser::parse_program;
use crate::neat::unit::resolve_unit;
use crate::neat::Namespace;

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
