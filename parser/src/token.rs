use std::fmt::{self};

/// Python source code can be tokenized in a sequence of these tokens.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Tok<'input> {
    Identifier(&'input str),
    StringLiteral(&'input str),

    Pragma,
    Import,
    As,

    NewLine,
    Semicolon,
}

impl<'input> fmt::Display for Tok<'input> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tok::Identifier(id) => write!(f, "{}", id),
            Tok::StringLiteral(s) => write!(f, "\"{}\"", s),
            Tok::Pragma => write!(f, "pragma"),
            Tok::Semicolon => write!(f, ";"),
            Tok::Import => write!(f, "import"),
            Tok::As => write!(f, "as"),
            Tok::NewLine => write!(f, "NEWLINE"),
        }
    }
}
