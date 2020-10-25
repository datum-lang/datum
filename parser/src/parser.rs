use crate::ast::Program;
use crate::charj;
use crate::error::ParseError;
use crate::lexer;

#[derive(Debug, PartialEq)]
pub enum Top {
    Program(Program),
}

macro_rules! do_lalr_parsing {
    ($input: expr, $pat: ident, $tok: ident) => {{
        // let lxr = lexer::make_tokenizer($input);
        // let marker_token = (Default::default(), token::Tok::$tok, Default::default());
        // let tokenizer = iter::once(Ok(marker_token)).chain(lxr);

        let lex = lexer::Lexer::new($input);
        match charj::CharjParser::new().parse($input, lex) {
            Err(err) => {
                println!("{}", err);
                // Err(ParseError::from(err));
            }
            Ok(top) => {
                println!("{}", top);
            }
        };
    }};
}

/// Parse a full charj program, containing usually multiple lines.
// pub fn parse_program(source: &str) -> Result<ast::Program, ParseError> {
//     do_lalr_parsing!(source, Program, StartProgram)
// }
pub fn parse_program(source: &str) {
    do_lalr_parsing!(source, Program, StartProgram);
}

#[cfg(test)]
mod test {
    use crate::parser::parse_program;

    #[test]
    fn test_parse_empty() {
        let parse_ast = parse_program("pkg \"chajr\"");
        // assert_eq!(parse_ast, Ok(ast::Program { statements: vec![] }))
    }
}
