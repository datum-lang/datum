use std::fmt::{self, Write};

/// Python source code can be tokenized in a sequence of these tokens.
#[derive(Clone, Debug, PartialEq)]
pub enum Tok {
    Name { name: String },
}

impl fmt::Display for Tok {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Tok::*;
        match self {
            Name { name } => write!(f, "'{}'", name),
        }
    }
}
