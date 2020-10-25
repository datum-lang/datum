use lalrpop_util::ParseError as LalrpopError;

use crate::location::Location;
use crate::token::Tok;
use std::fmt;

/// Represents an error during parsing
#[derive(Debug, PartialEq)]
pub struct ParseError<'input> {
    pub error: ParseErrorType<'input>,
    pub location: Location,
}

/// Convert `lalrpop_util::ParseError` to our internal type
impl<'input> From<LalrpopError<Location, Tok<'input>, LexicalError>> for ParseError<'input> {
    fn from(err: LalrpopError<Location, Tok<'input>, LexicalError>) -> Self {
        match err {
            _ => {
                ParseError {
                    error: ParseErrorType::EOF,
                    location: Default::default()
                }
            }
        }
    }
}

impl<'input> fmt::Display for ParseError<'input> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} at {}", self.error, self.location)
    }
}

/// Represents an error during lexical scanning.
// #[derive(Debug, PartialEq)]
// pub struct LexicalError {
//     pub error: LexicalErrorType,
//     pub location: Location,
// }
//
// impl fmt::Display for LexicalError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             _ => write!(f, "default"),
//         }
//     }
// }


pub enum LexicalError {
    EndOfFileInComment(usize, usize),
    EndOfFileInString(usize, usize),
    EndofFileInHex(usize, usize),
    MissingNumber(usize, usize),
    InvalidCharacterInHexLiteral(usize, char),
    UnrecognisedToken(usize, usize, String),
    MissingExponent(usize, usize),
    ExpectedFrom(usize, usize, String),
}

impl fmt::Display for LexicalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexicalError::EndOfFileInComment(_, _) => write!(f, "end of file found in comment"),
            LexicalError::EndOfFileInString(_, _) => {
                write!(f, "end of file found in string literal")
            }
            LexicalError::EndofFileInHex(_, _) => {
                write!(f, "end of file found in hex literal string")
            }
            LexicalError::MissingNumber(_, _) => write!(f, "missing number"),
            LexicalError::InvalidCharacterInHexLiteral(_, ch) => {
                write!(f, "invalid character ‘{}’ in hex literal string", ch)
            }
            LexicalError::UnrecognisedToken(_, _, t) => write!(f, "unrecognised token ‘{}’", t),
            LexicalError::ExpectedFrom(_, _, t) => write!(f, "‘{}’ found where ‘from’ expected", t),
            LexicalError::MissingExponent(_, _) => write!(f, "missing number"),
        }
    }
}

impl LexicalError {
    pub fn loc(&self, file_no: usize) -> Location {
        match self {
            LexicalError::EndOfFileInComment(start, end) => Location::new(*start, *end),
            LexicalError::EndOfFileInString(start, end) => Location::new(*start, *end),
            LexicalError::EndofFileInHex(start, end) => Location::new(*start, *end),
            LexicalError::MissingNumber(start, end) => Location::new(*start, *end),
            LexicalError::InvalidCharacterInHexLiteral(pos, _) => Location::new(*pos, *pos),
            LexicalError::UnrecognisedToken(start, end, _) => Location::new(*start, *end),
            LexicalError::ExpectedFrom(start, end, _) => Location::new(*start, *end),
            LexicalError::MissingExponent(start, end) => Location::new(*start, *end),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseErrorType<'input> {
    /// Parser encountered an unexpected end of input
    EOF,
    /// Parser encountered an extra token
    ExtraToken(Tok<'input>),
    /// Parser encountered an invalid token
    InvalidToken,
    /// Parser encountered an unexpected token
    UnrecognizedToken(Tok<'input>, Option<String>),
    /// Maps to `User` type from `lalrpop-util`
    Lexical(LexicalErrorType),
}

impl<'input> fmt::Display for ParseErrorType<'input> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseErrorType::EOF => write!(f, "Got unexpected EOF"),
            ParseErrorType::ExtraToken(ref tok) => write!(f, "Got extraneous token: {:?}", tok),
            ParseErrorType::InvalidToken => write!(f, "Got invalid token"),
            ParseErrorType::UnrecognizedToken(ref tok, ref expected) => {
                // if *tok == Tok::Indent {
                //     write!(f, "unexpected indent")
                // } else if expected.as_deref() == Some("Indent") {
                //     write!(f, "expected an indented block")
                // } else {
                //     write!(f, "Got unexpected token {}", tok)
                // }
                write!(f, "UnrecognizedToken")
            }
            ParseErrorType::Lexical(ref error) => write!(f, "{}", error),
        }
    }
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

impl fmt::Display for LexicalErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LexicalErrorType::StringError => write!(f, "Got unexpected string"),
            // LexicalErrorType::FStringError(error) => write!(f, "Got error in f-string: {}", error),
            LexicalErrorType::UnicodeError => write!(f, "Got unexpected unicode"),
            LexicalErrorType::NestingError => write!(f, "Got unexpected nesting"),
            LexicalErrorType::IndentationError => {
                write!(f, "unindent does not match any outer indentation level")
            }
            LexicalErrorType::TabError => {
                write!(f, "inconsistent use of tabs and spaces in indentation")
            }
            LexicalErrorType::DefaultArgumentError => {
                write!(f, "non-default argument follows default argument")
            }
            LexicalErrorType::DuplicateKeywordArgumentError => {
                write!(f, "keyword argument repeated")
            }
            LexicalErrorType::PositionalArgumentError => {
                write!(f, "positional argument follows keyword argument")
            }
            LexicalErrorType::UnrecognizedToken { tok } => {
                write!(f, "Got unexpected token {}", tok)
            }
            LexicalErrorType::LineContinuationError => {
                write!(f, "unexpected character after line continuation character")
            }
            LexicalErrorType::EOF => write!(f, "unexpected EOF while parsing"),
            LexicalErrorType::OtherError(msg) => write!(f, "{}", msg),
        }
    }
}
