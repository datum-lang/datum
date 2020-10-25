use crate::charj;
use crate::lexer;
use crate::error::ParseErrorType;

macro_rules! do_lalr_parsing {
    ($input: expr, $file_no: ident) => {{
        let lex = lexer::Lexer::new($input);
        match charj::CharjParser::new().parse($input, $file_no, lex) {
            Err(err) => {
                println!("{:?}", err);
            }
            Ok(top) => {
                println!("{:?}", top);
            }
        };
    }};
}

pub fn parse_program(source: &str, file_no: usize) {
    do_lalr_parsing!(source, file_no);
}

#[cfg(test)]
mod test {
    use crate::parser::parse_program;

    #[test]
    fn test_parse_empty() {
        let parse_ast = parse_program("pkg \"chajr\"", 0);
        // assert_eq!(parse_ast, Ok(ast::Program { statements: vec![] }))
    }
}
