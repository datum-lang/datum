use crate::charj;
use crate::error::Diagnostic;
use crate::lexer;
use crate::parse_tree::SourceUnit;

macro_rules! do_lalr_parsing {
    ($input: expr, $file_no: ident) => {{
        let lex = lexer::Lexer::new($input);
        match charj::CharjParser::new().parse($input, $file_no, lex) {
            Err(err) => Err(Diagnostic::handle_error(err)),
            Ok(s) => Ok(s),
        }
    }};
}

pub fn parse_program(source: &str, file_no: usize) -> Result<SourceUnit, Diagnostic> {
    do_lalr_parsing!(source, file_no)
}

#[cfg(test)]
mod test {
    use crate::parse_tree::{Import, SourceUnitPart};
    use crate::parser::parse_program;

    #[test]
    #[rustfmt::skip]
    fn test_parse_empty() {
        let parse_ast = parse_program("", 0);
        assert!(parse_ast.is_err());

        // let message = String::from("unexpected end of file, expected \"import\"");
        // assert_eq!(parse_ast, Err(Diagnostic::parser_error(Loc(0, 0, 0), message)));
    }

    #[test]
    #[rustfmt::skip]
    fn test_parse_package() {
        let package = parse_program("package charj", 0);
        assert!(package.is_ok());
        let pkg_alias = parse_program("pkg charj", 0);
        assert!(pkg_alias.is_ok());
    }

    #[test]
    #[rustfmt::skip]
    fn test_parse_struct() {
        let package = parse_program("struct IO {}", 0);
        assert!(package.is_ok());
    }

    #[test]
    #[rustfmt::skip]
    fn test_basic_location() {
        let code = parse_program("pkg charj
struct IO {}", 0);
        println!("{:?}", code);
        assert!(code.is_ok());
    }

    #[test]
    #[rustfmt::skip]
    fn test_normal_struct_function() {
        let normal_struct_fun = parse_program("default$main() {}", 0);
        assert!(normal_struct_fun.is_ok());
        let with_empty_struct_fun = parse_program("default $ main () {}", 0);
        assert!(with_empty_struct_fun.is_ok());
    }

    #[test]
    #[rustfmt::skip]
    fn test_function_parameters() {
        let params = parse_program("default$main(string name) {}", 0);
        assert!(params.is_ok());

        let multi_params = parse_program("default$main(string name, string first, int id) {}", 0);
        assert!(multi_params.is_ok());
    }

    #[test]
    #[rustfmt::skip]
    fn test_comment() {
        let comments = parse_program("// this is a comment
pkg comment
", 0);
        assert!(comments.is_ok());
    }

    #[test]
    #[rustfmt::skip]
    fn test_if_statement() {
        let empty_if = parse_program("default$main(string name) {
if() {}

while() {}
}", 0);
        assert!(empty_if.is_ok());

        let if_with_expr = parse_program("default$main(string name) {
    if( a == true) {}
}", 0);
        println!("{:?}", if_with_expr);
    }

    #[test]
    #[rustfmt::skip]
    fn test_parse_import() {
        let parse_ast = parse_program("import io", 0);
        assert!(parse_ast.is_ok());

        if let SourceUnitPart::ImportDirective(import) = parse_ast.unwrap().0.get(0).unwrap() {
            if let Import::Standard(plain) = import {
                assert_eq!("io", plain.name);
            } else {
                panic!("error");
            }
        } else {
            panic!("error");
        }
    }
}
