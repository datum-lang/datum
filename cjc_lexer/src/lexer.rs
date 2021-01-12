use std::iter::Peekable;
use std::str::CharIndices;

use phf::phf_map;
use unicode_xid::UnicodeXID;

use crate::error::LexicalError;
use crate::location::Location;
use crate::token::{CommentType, Token};

#[allow(unused)]
pub struct Lexer<'input> {
    input: &'input str,
    chars: Peekable<CharIndices<'input>>,
    last_tokens: [Option<Token<'input>>; 2],
    char0: Option<char>,
    location: Location,
}

static KEYWORDS: phf::Map<&'static str, Token> = phf_map! {
    // defalut
    "import"  => Token::Import,
    "package" => Token::Package,
    "pkg" => Token::Package,
    "struct" => Token::Struct,
    "as" => Token::As,
    "fun" => Token::Fun,

    // statement
    "if" => Token::If,
    "else" => Token::Else,
    "while" => Token::While,
    "for" => Token::For,
    "in" => Token::In,
    "break" => Token::Break,
    "continue" => Token::Continue,
    "return" => Token::Return,
    "let" => Token::Let,

    // type
    "bool" => Token::Bool,
    "true" => Token::True,
    "false" => Token::False,

    "string" => Token::String,
    "int" => Token::Int(256),
    // "int256" => Token::Int(256),
    "uint" => Token::Uint(256),

    "$" => Token::Binding,
};

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        let mut lexer = Lexer {
            input,
            chars: input.char_indices().peekable(),
            last_tokens: [None, None],
            char0: None,
            location: Location::new(0, 0),
        };

        lexer.location.reset();
        lexer
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

    /// Helper function to go to the next character coming up.
    #[allow(unused)]
    fn next_char(&mut self) -> Option<char> {
        let next_char = self.char0;
        if next_char == Some('\n') {
            self.location.newline();
        } else {
            self.location.go_right();
        }
        next_char
    }

    fn parse_number(
        &mut self,
        start: usize,
        end: usize,
        ch: char,
    ) -> Option<Result<(usize, Token<'input>, usize), LexicalError>> {
        if ch == '0' {
            if let Some((_, 'x')) = self.chars.peek() {
                // hex number
                self.chars.next();

                let mut end = match self.chars.next() {
                    Some((end, ch)) if ch.is_ascii_hexdigit() => end,
                    Some((_, _)) => {
                        return Some(Err(LexicalError::MissingNumber(start, start + 1)));
                    }
                    None => {
                        return Some(Err(LexicalError::EndOfFileInHex(start, self.input.len())));
                    }
                };

                while let Some((i, ch)) = self.chars.peek() {
                    if !ch.is_ascii_hexdigit() && *ch != '_' {
                        break;
                    }
                    end = *i;
                    self.chars.next();
                }

                return Some(Ok((
                    start,
                    Token::HexNumber(&self.input[start..=end]),
                    end + 1,
                )));
            }
        }

        let mut end = end;
        while let Some((i, ch)) = self.chars.peek() {
            if !ch.is_ascii_digit() && *ch != '_' {
                break;
            }
            end = *i;
            self.chars.next();
        }

        let base = &self.input[start..=end];

        let mut exp_start = end + 1;

        if let Some((i, 'e')) = self.chars.peek() {
            exp_start = i + 1;
            self.chars.next();
            while let Some((i, ch)) = self.chars.peek() {
                if !ch.is_ascii_digit() && *ch != '_' {
                    break;
                }
                end = *i;
                self.chars.next();
            }

            if exp_start > end {
                return Some(Err(LexicalError::MissingExponent(start, self.input.len())));
            }
        }

        let exp = &self.input[exp_start..=end];

        Some(Ok((start, Token::NumberLiteral(base, exp), end + 1)))
    }

    fn next(&mut self) -> Option<Result<(usize, Token<'input>, usize), LexicalError>> {
        loop {
            match self.chars.next() {
                Some((start, ch)) if ch == '_' || UnicodeXID::is_xid_start(ch) => {
                    let end;

                    loop {
                        if let Some((i, ch)) = self.chars.peek() {
                            if !UnicodeXID::is_xid_continue(*ch) {
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
                Some((start, '/')) => {
                    match self.chars.peek() {
                        Some((_, '=')) => {
                            self.chars.next();
                            return Some(Ok((start, Token::DivideAssign, start + 2)));
                        }
                        Some((_, '/')) => {
                            // line comment
                            self.chars.next();

                            let doc_comment_start = match self.chars.peek() {
                                Some((i, '/')) => Some(i + 1),
                                _ => None,
                            };

                            let mut last = start + 3;

                            while let Some((i, ch)) = self.chars.next() {
                                if ch == '\n' || ch == '\r' {
                                    break;
                                }
                                last = i;
                            }

                            if let Some(doc_start) = doc_comment_start {
                                if last > doc_start {
                                    return Some(Ok((
                                        start + 3,
                                        Token::DocComment(
                                            CommentType::Line,
                                            &self.input[doc_start..=last],
                                        ),
                                        last + 1,
                                    )));
                                }
                            }
                        }
                        Some((_, '*')) => {
                            // multiline comment
                            self.chars.next();

                            let doc_comment_start = match self.chars.peek() {
                                Some((i, '*')) => Some(i + 1),
                                _ => None,
                            };

                            let mut last = start + 3;
                            let mut seen_star = false;

                            loop {
                                if let Some((i, ch)) = self.chars.next() {
                                    if seen_star && ch == '/' {
                                        break;
                                    }
                                    seen_star = ch == '*';
                                    last = i;
                                } else {
                                    return Some(Err(LexicalError::EndOfFileInComment(
                                        start,
                                        self.input.len(),
                                    )));
                                }
                            }

                            if let Some(doc_start) = doc_comment_start {
                                if last > doc_start {
                                    return Some(Ok((
                                        start + 3,
                                        Token::DocComment(
                                            CommentType::Block,
                                            &self.input[doc_start..last],
                                        ),
                                        last,
                                    )));
                                }
                            }
                        }
                        _ => {
                            return Some(Ok((start, Token::Divide, start + 1)));
                        }
                    }
                }
                Some((_, ch)) if ch.is_whitespace() => (),
                Some((start, ch)) if ch.is_ascii_digit() => {
                    return self.parse_number(start, start, ch)
                }
                Some((i, ';')) => return Some(Ok((i, Token::Semicolon, i + 1))),
                Some((i, ',')) => return Some(Ok((i, Token::Comma, i + 1))),
                Some((i, '(')) => return Some(Ok((i, Token::OpenParenthesis, i + 1))),
                Some((i, ')')) => return Some(Ok((i, Token::CloseParenthesis, i + 1))),
                Some((i, '{')) => return Some(Ok((i, Token::OpenCurlyBrace, i + 1))),
                Some((i, '}')) => return Some(Ok((i, Token::CloseCurlyBrace, i + 1))),
                Some((i, '~')) => return Some(Ok((i, Token::Complement, i + 1))),
                Some((i, '=')) => {
                    return match self.chars.peek() {
                        Some((_, '=')) => {
                            self.chars.next();
                            Some(Ok((i, Token::Equal, i + 2)))
                        }
                        Some((_, '>')) => {
                            self.chars.next();
                            Some(Ok((i, Token::Arrow, i + 2)))
                        }
                        _ => Some(Ok((i, Token::Assign, i + 1))),
                    };
                }
                Some((i, '!')) => {
                    if let Some((_, '=')) = self.chars.peek() {
                        self.chars.next();
                        return Some(Ok((i, Token::NotEqual, i + 2)));
                    } else {
                        return Some(Ok((i, Token::Not, i + 1)));
                    }
                }
                Some((i, '|')) => {
                    return match self.chars.peek() {
                        Some((_, '=')) => {
                            self.chars.next();
                            Some(Ok((i, Token::BitwiseOrAssign, i + 2)))
                        }
                        Some((_, '|')) => {
                            self.chars.next();
                            Some(Ok((i, Token::Or, i + 2)))
                        }
                        _ => Some(Ok((i, Token::BitwiseOr, i + 1))),
                    };
                }
                Some((i, '&')) => {
                    return match self.chars.peek() {
                        Some((_, '=')) => {
                            self.chars.next();
                            Some(Ok((i, Token::BitwiseAndAssign, i + 2)))
                        }
                        Some((_, '&')) => {
                            self.chars.next();
                            Some(Ok((i, Token::And, i + 2)))
                        }
                        _ => Some(Ok((i, Token::BitwiseAnd, i + 1))),
                    };
                }
                Some((i, '+')) => {
                    return match self.chars.peek() {
                        Some((_, '=')) => {
                            self.chars.next();
                            Some(Ok((i, Token::AddAssign, i + 2)))
                        }
                        Some((_, '+')) => {
                            self.chars.next();
                            Some(Ok((i, Token::Increment, i + 2)))
                        }
                        _ => Some(Ok((i, Token::Add, i + 1))),
                    };
                }
                Some((i, '-')) => {
                    return match self.chars.peek() {
                        Some((_, '=')) => {
                            self.chars.next();
                            Some(Ok((i, Token::SubtractAssign, i + 2)))
                        }
                        Some((_, '>')) => {
                            self.chars.next();
                            Some(Ok((i, Token::Rarrow, i + 2)))
                        }
                        Some((_, '-')) => {
                            self.chars.next();
                            Some(Ok((i, Token::Decrement, i + 2)))
                        }
                        _ => Some(Ok((i, Token::Subtract, i + 1))),
                    };
                }
                Some((i, '*')) => {
                    return match self.chars.peek() {
                        Some((_, '=')) => {
                            self.chars.next();
                            Some(Ok((i, Token::MulAssign, i + 2)))
                        }
                        Some((_, '*')) => {
                            self.chars.next();
                            Some(Ok((i, Token::Power, i + 2)))
                        }
                        _ => Some(Ok((i, Token::Mul, i + 1))),
                    };
                }
                Some((i, '%')) => {
                    return match self.chars.peek() {
                        Some((_, '=')) => {
                            self.chars.next();
                            Some(Ok((i, Token::ModuloAssign, i + 2)))
                        }
                        _ => Some(Ok((i, Token::Modulo, i + 1))),
                    };
                }
                Some((i, '<')) => {
                    return match self.chars.peek() {
                        Some((_, '<')) => {
                            self.chars.next();
                            if let Some((_, '=')) = self.chars.peek() {
                                self.chars.next();
                                Some(Ok((i, Token::ShiftLeftAssign, i + 3)))
                            } else {
                                Some(Ok((i, Token::ShiftLeft, i + 2)))
                            }
                        }
                        Some((_, '=')) => {
                            self.chars.next();
                            Some(Ok((i, Token::LessEqual, i + 2)))
                        }
                        _ => Some(Ok((i, Token::Less, i + 1))),
                    };
                }
                Some((i, '>')) => {
                    return match self.chars.peek() {
                        Some((_, '>')) => {
                            self.chars.next();
                            if let Some((_, '=')) = self.chars.peek() {
                                self.chars.next();
                                Some(Ok((i, Token::ShiftRightAssign, i + 3)))
                            } else {
                                Some(Ok((i, Token::ShiftRight, i + 2)))
                            }
                        }
                        Some((_, '=')) => {
                            self.chars.next();
                            Some(Ok((i, Token::MoreEqual, i + 2)))
                        }
                        _ => Some(Ok((i, Token::More, i + 1))),
                    };
                }
                Some((i, '.')) => {
                    return match self.chars.peek() {
                        Some((_, '.')) => {
                            self.chars.next();
                            Some(Ok((i, Token::Range, i + 2)))
                        }
                        _ => Some(Ok((i, Token::Member, i + 1))),
                    };
                }
                Some((i, '[')) => return Some(Ok((i, Token::OpenBracket, i + 1))),
                Some((i, ']')) => return Some(Ok((i, Token::CloseBracket, i + 1))),
                Some((i, ':')) => return Some(Ok((i, Token::Colon, i + 1))),
                Some((i, '?')) => return Some(Ok((i, Token::Question, i + 1))),
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
