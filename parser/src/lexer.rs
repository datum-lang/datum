use crate::token::Tok;
use std::iter::Peekable;
use std::str::CharIndices;
use crate::error::LexicalError;

pub struct Lexer<'input> {
    input: &'input str,
    chars: Peekable<CharIndices<'input>>,
    last_tokens: [Option<Tok<'input>>; 2],
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer {
            input,
            chars: input.char_indices().peekable(),
            last_tokens: [None, None],
        }
    }
}

pub type Spanned<Token, Loc, Error> = Result<(Loc, Token, Loc), Error>;

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Tok<'input>, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}