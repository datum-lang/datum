use std::fmt::{self};

/// Python source code can be tokenized in a sequence of these tokens.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Tok<'input> {
    LexIdentifier(&'input str),
    LexStringLiteral(&'input str),
    Import,
    Semicolon,
    As
}

impl<'input> fmt::Display for Tok<'input> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Tok::*;
        match self {
            LexIdentifier(id) => write!(f, "{}", id),
            LexStringLiteral(s) => write!(f, "\"{}\"", s),
            Semicolon => write!(f, ";"),
            Import => write!(f, "import"),
            As => write!(f, "as")
        }
    }
}
