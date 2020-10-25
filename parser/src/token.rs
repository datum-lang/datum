use std::fmt::{self};

/// Python source code can be tokenized in a sequence of these tokens.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Tok<'input> {
    Identifier(&'input str),
    // excludes the `"`
    StringLiteral(&'input str),
    // excludes the `"`
    Import,
    Semicolon,
}

impl<'input> fmt::Display for Tok<'input> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Tok::*;
        match self {
            Identifier(id) => write!(f, "{}", id),
            StringLiteral(s) => write!(f, "\"{}\"", s),
            Semicolon => write!(f, ";"),
            Import => write!(f, "import"),
        }
    }
}
