use std::iter::Peekable;
use std::str::CharIndices;

use phf::phf_map;
use unicode_xid::UnicodeXID;

use crate::error::LexicalError;
use crate::token::Token;

pub struct Lexer<'input> {
    input: &'input str,
    chars: Peekable<CharIndices<'input>>,
    last_tokens: [Option<Token<'input>>; 2],
}

static KEYWORDS: phf::Map<&'static str, Token> = phf_map! {
    "import"  => Token::Import,
    "package" => Token::Package,
    "pkg" => Token::Package,
    "struct" => Token::Struct,
    "$" => Token::Binding,
};

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer {
            input,
            chars: input.char_indices().peekable(),
            last_tokens: [None, None],
        }
    }

    fn lex_string(
        &mut self,
        token_start: usize,
        string_start: usize,
    ) -> Option<Result<(usize, Token<'input>, usize), LexicalError>> {
        let mut end;

        let mut last_was_escape = false;

        loop {
            if let Some((i, ch)) = self.chars.next() {
                end = i;
                if !last_was_escape {
                    if ch == '"' {
                        break;
                    }
                    last_was_escape = ch == '\\';
                } else {
                    last_was_escape = false;
                }
            } else {
                return Some(Err(LexicalError::EndOfFileInString(
                    token_start,
                    self.input.len(),
                )));
            }
        }

        Some(Ok((
            token_start,
            Token::StringLiteral(&self.input[string_start..end]),
            end + 1,
        )))
    }

    fn next(&mut self) -> Option<Result<(usize, Token<'input>, usize), LexicalError>> {
        loop {
            match self.chars.next() {
                Some((start, ch)) if ch == '_' || ch == '$' || UnicodeXID::is_xid_start(ch) => {
                    let end;

                    loop {
                        if let Some((i, ch)) = self.chars.peek() {
                            if !UnicodeXID::is_xid_continue(*ch) && *ch != '$' {
                                end = *i;
                                break;
                            }
                            self.chars.next();
                        } else {
                            end = self.input.len();
                            break;
                        }
                    }

                    let id = &self.input[start..end];

                    if id == "unicode" {
                        if let Some((_, '"')) = self.chars.peek() {
                            self.chars.next();

                            return self.lex_string(start, start + 8);
                        }
                    }

                    if id == "hex" {
                        if let Some((_, '"')) = self.chars.peek() {
                            self.chars.next();

                            while let Some((i, ch)) = self.chars.next() {
                                if ch == '"' {
                                    return Some(Ok((
                                        start,
                                        Token::HexLiteral(&self.input[start..=i]),
                                        i + 1,
                                    )));
                                }

                                if !ch.is_ascii_hexdigit() && ch != '_' {
                                    // Eat up the remainer of the string
                                    while let Some((_, ch)) = self.chars.next() {
                                        if ch == '"' {
                                            break;
                                        }
                                    }

                                    return Some(Err(LexicalError::InvalidCharacterInHexLiteral(
                                        i, ch,
                                    )));
                                }
                            }

                            return Some(Err(LexicalError::EndOfFileInString(
                                start,
                                self.input.len(),
                            )));
                        }
                    }

                    return if let Some(w) = KEYWORDS.get(id) {
                        Some(Ok((start, *w, end)))
                    } else {
                        Some(Ok((start, Token::Identifier(id), end)))
                    };
                }
                Some((start, '"')) => {
                    return self.lex_string(start, start + 1);
                }
                Some((_, ch)) if ch.is_whitespace() => (),
                Some((i, ';')) => return Some(Ok((i, Token::Semicolon, i + 1))),
                Some((i, ',')) => return Some(Ok((i, Token::Comma, i + 1))),
                Some((i, '(')) => return Some(Ok((i, Token::OpenParenthesis, i + 1))),
                Some((i, ')')) => return Some(Ok((i, Token::CloseParenthesis, i + 1))),
                Some((i, '{')) => return Some(Ok((i, Token::OpenCurlyBrace, i + 1))),
                Some((i, '}')) => return Some(Ok((i, Token::CloseCurlyBrace, i + 1))),
                Some((i, '.')) => return Some(Ok((i, Token::Member, i + 1))),
                Some((i, '[')) => return Some(Ok((i, Token::OpenBracket, i + 1))),
                Some((i, ']')) => return Some(Ok((i, Token::CloseBracket, i + 1))),
                Some((i, ':')) => return Some(Ok((i, Token::Colon, i + 1))),
                Some((i, '?')) => return Some(Ok((i, Token::Question, i + 1))),
                Some((i, '~')) => return Some(Ok((i, Token::Complement, i + 1))),

                Some((i, '$')) => return Some(Ok((i, Token::Binding, i + 1))),
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
    type Item = Spanned<Token<'input>, usize, LexicalError>;

    /// Return the next token
    fn next(&mut self) -> Option<Self::Item> {
        let token = self.next();

        self.last_tokens = [
            self.last_tokens[1],
            match token {
                Some(Ok((_, n, _))) => Some(n),
                _ => None,
            },
        ];

        token
    }
}
