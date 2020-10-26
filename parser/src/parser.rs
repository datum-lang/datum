use crate::charj;
use crate::error::Diagnostic;
use crate::lexer;
use crate::pt::SourceUnit;

macro_rules! do_lalr_parsing {
    ($input: expr, $file_no: ident) => {{
        let lex = lexer::Lexer::new($input);
        println!("{:?}", lex.clone());
        match charj::CharjParser::new().parse($input, $file_no, lex) {
            Err(err) => Err(Diagnostic::handle_error($file_no, err)),
            Ok(s) => Ok(s),
        }
    }};
}

pub fn parse_program(source: &str, file_no: usize) -> Result<SourceUnit, Diagnostic> {
    do_lalr_parsing!(source, file_no)
}

#[cfg(test)]
mod test {
    use crate::error::Diagnostic;
    use crate::location::Loc;
    use crate::parser::parse_program;

    #[test]
    #[rustfmt::skip]
    fn test_parse_empty() {
        let parse_ast = parse_program("", 0);
        assert!(parse_ast.is_err());
        println!("{:?}", parse_ast);

        let message = String::from("unexpected end of file, expected \"import\"");
        assert_eq!(parse_ast, Err(Diagnostic::parser_error(Loc(0, 0, 0), message)));
    }

    #[test]
    #[rustfmt::skip]
    fn test_parse_import() {
        let parse_ast = parse_program("import hello;", 0);
        assert!(parse_ast.is_err());
        println!("{:?}", parse_ast);

        let message = String::from("unexpected end of file, expected \"import\"");
        assert_eq!(parse_ast, Err(Diagnostic::parser_error(Loc(0, 0, 0), message)));
    }
}
