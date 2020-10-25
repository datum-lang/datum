use crate::location::Location;
use crate::token::Tok;

/// Represents an error during parsing
#[derive(Debug, PartialEq)]
pub struct ParseError {
    pub error: ParseErrorType,
    pub location: Location,
}

#[derive(Debug, PartialEq)]
pub enum ParseErrorType {
    /// Parser encountered an unexpected end of input
    EOF,
    /// Parser encountered an extra token
    ExtraToken(Tok),
    /// Parser encountered an invalid token
    InvalidToken,
    /// Parser encountered an unexpected token
    UnrecognizedToken(Tok, Option<String>),
    /// Maps to `User` type from `lalrpop-util`
    Lexical(LexicalErrorType),
}

#[derive(Debug, PartialEq)]
pub enum LexicalErrorType {
    StringError,
    UnicodeError,
    NestingError,
    IndentationError,
    TabError,
    DefaultArgumentError,
    PositionalArgumentError,
    DuplicateKeywordArgumentError,
    UnrecognizedToken { tok: char },
    // FStringError(FStringErrorType),
    LineContinuationError,
    EOF,
    OtherError(String),
}
