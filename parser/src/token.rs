use std::fmt::{self};

/// Python source code can be tokenized in a sequence of these tokens.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Token<'input> {
    Identifier(&'input str),
    StringLiteral(&'input str),
    HexLiteral(&'input str),

    Pragma,
    Package,
    Import,
    Struct,
    As,

    NewLine,
    Semicolon,
    Comma,
    OpenParenthesis,
    CloseParenthesis,
    OpenCurlyBrace,
    CloseCurlyBrace,
}

impl<'input> fmt::Display for Token<'input> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Identifier(id) => write!(f, "{}", id),
            Token::StringLiteral(s) => write!(f, "\"{}\"", s),
            Token::HexLiteral(hex) => write!(f, "{}", hex),

            Token::Pragma => write!(f, "pragma"),
            Token::Package => write!(f, "package"),
            Token::Import => write!(f, "import"),
            Token::Struct => write!(f, "struct"),

            Token::Semicolon => write!(f, ";"),
            Token::Comma => write!(f, ","),
            Token::OpenParenthesis => write!(f, "("),
            Token::CloseParenthesis => write!(f, ")"),
            Token::OpenCurlyBrace => write!(f, "{{"),
            Token::CloseCurlyBrace => write!(f, "}}"),
            Token::As => write!(f, "as"),
            Token::NewLine => write!(f, "NEWLINE"),
        }
    }
}
