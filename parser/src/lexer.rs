use crate::error::LexicalError;
use crate::token::Tok;
use std::iter::Peekable;
use std::str::CharIndices;

#[derive(Debug, Clone)]
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

    fn next(&mut self) -> Option<Result<(usize, Tok<'input>, usize), LexicalError>> {
        loop {
            match self.chars.next() {
                Some((_, ch)) if ch.is_whitespace() => (),
                Some((i, ';')) => return Some(Ok((i, Tok::Semicolon, i + 1))),
                Some((start, _)) => {
                    let mut end;

                    loop {
                        if let Some((i, ch)) = self.chars.next() {
                            end = i;

                            if ch.is_whitespace() {
                                break;
                            }
                        } else {
                            end = self.input.len();
                            break;
                        }
                    }

                    return Some(Err(LexicalError::UnrecognisedToken(
                        start,
                        end,
                        self.input[start..end].to_owned(),
                    )));
                }
                None => return None, // End of file
            }
        }
    }
}

pub type Spanned<Token, Loc, Error> = Result<(Loc, Token, Loc), Error>;

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Tok<'input>, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        // todo: add next logic
        let option = self.next();
        println!("{:?}", option);
        None
    }
}
