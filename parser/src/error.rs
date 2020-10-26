use std::fmt;

use lalrpop_util::ParseError;

use crate::location::Loc;
use crate::token::Token;

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Level {
    Debug,
    Info,
    Warning,
    Error,
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum ErrorType {
    None,
    ParserError,
    SyntaxError,
    DeclarationError,
    TypeError,
    Warning,
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Note {
    pub pos: Loc,
    pub message: String,
}

#[derive(Debug, PartialEq)]
pub struct Diagnostic {
    pub level: Level,
    pub ty: ErrorType,
    pub pos: Option<Loc>,
    pub message: String,
    pub notes: Vec<Note>,
}

impl Diagnostic {
    pub fn handle_error(
        file_no: usize,
        error: ParseError<usize, Token, LexicalError>,
    ) -> Diagnostic {
        match error {
            ParseError::InvalidToken { location } => Diagnostic::parser_error(
                Loc(file_no, location, location),
                "invalid token".to_string(),
            ),
            ParseError::UnrecognizedToken {
                token: (l, token, r),
                expected,
            } => Diagnostic::parser_error(
                Loc(file_no, l, r),
                format!(
                    "unrecognised token `{}', expected {}",
                    token,
                    expected.join(", ")
                ),
            ),
            ParseError::User { error } => {
                Diagnostic::parser_error(error.loc(file_no), error.to_string())
            }
            ParseError::ExtraToken { token } => Diagnostic::parser_error(
                Loc(file_no, token.0, token.2),
                format!("extra token `{}' encountered", token.0),
            ),
            ParseError::UnrecognizedEOF { location, expected } => Diagnostic::parser_error(
                Loc(file_no, location, location),
                format!("unexpected end of file, expected {}", expected.join(", ")),
            ),
        }
    }

    pub fn error(pos: Loc, message: String) -> Self {
        Diagnostic {
            level: Level::Error,
            ty: ErrorType::SyntaxError,
            pos: Some(pos),
            message,
            notes: Vec::new(),
        }
    }

    pub fn parser_error(pos: Loc, message: String) -> Self {
        Diagnostic {
            level: Level::Error,
            ty: ErrorType::ParserError,
            pos: Some(pos),
            message,
            notes: Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum LexicalError {
    EndOfFileInComment(usize, usize),
    EndOfFileInString(usize, usize),
    EndOfFileInHex(usize, usize),
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
            LexicalError::EndOfFileInHex(_, _) => {
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
    pub fn loc(&self, file_no: usize) -> Loc {
        match self {
            LexicalError::EndOfFileInComment(start, end) => Loc(file_no, *start, *end),
            LexicalError::EndOfFileInString(start, end) => Loc(file_no, *start, *end),
            LexicalError::EndOfFileInHex(start, end) => Loc(file_no, *start, *end),
            LexicalError::MissingNumber(start, end) => Loc(file_no, *start, *end),
            LexicalError::InvalidCharacterInHexLiteral(pos, _) => Loc(file_no, *pos, *pos),
            LexicalError::UnrecognisedToken(start, end, _) => Loc(file_no, *start, *end),
            LexicalError::ExpectedFrom(start, end, _) => Loc(file_no, *start, *end),
            LexicalError::MissingExponent(start, end) => Loc(file_no, *start, *end),
        }
    }
}
