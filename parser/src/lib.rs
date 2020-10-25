#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(unused)]
    pub charj
); // synthesized by LALRPOP

pub mod ast;
pub mod error;
pub mod lexer;
pub mod location;
pub mod parser;
pub mod token;

#[test]
fn test_charj() {
    assert!(charj::CharjParser::new().parse("22").is_ok());
    assert!(charj::CharjParser::new().parse("(22)").is_ok());
    assert!(charj::CharjParser::new().parse("((((22))))").is_ok());
    assert!(charj::CharjParser::new().parse("((22)").is_err());
}
