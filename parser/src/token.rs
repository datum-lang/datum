use std::fmt::{self};

/// Python source code can be tokenized in a sequence of these tokens.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Token<'input> {
    Identifier(&'input str),
    StringLiteral(&'input str),
    NumberLiteral(&'input str, &'input str),
    HexLiteral(&'input str),
    HexNumber(&'input str),

    DocComment(CommentType, &'input str),

    Package,
    Import,
    Struct,
    Default,
    As,
    Fun,
    If,
    Else,
    While,
    For,
    In,
    Range,
    Break,
    Continue,
    Return,
    Let,

    // type
    Bool,
    String,
    Uint(u16),
    Int(u16),
    Bytes(u8),
    DynamicBytes,

    NewLine,
    Binding,
    OpenParenthesis,
    CloseParenthesis,
    OpenCurlyBrace,
    CloseCurlyBrace,

    Question,
    Colon,

    NotEqual,
    Less,
    Arrow,
    LessEqual,
    More,
    MoreEqual,
    BitwiseXor,

    Or,
    BitwiseOr,
    BitwiseOrAssign,

    And,
    BitwiseAnd,
    BitwiseAndAssign,

    ShiftLeft,
    ShiftLeftAssign,

    ShiftRight,
    ShiftRightAssign,

    Add,
    AddAssign,
    Increment,

    Subtract,
    SubtractAssign,
    Decrement,

    BitwiseXorAssign,
    ModuloAssign,

    Power,
    Mul,
    MulAssign,

    Divide,
    DivideAssign,

    Modulo,
    Not,
    Complement,
    OpenBracket,
    CloseBracket,
    Member,
    Comma,
    Equal,
    Assign,
    Semicolon,
}

impl<'input> fmt::Display for Token<'input> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Identifier(id) => write!(f, "{}", id),
            Token::StringLiteral(s) => write!(f, "\"{}\"", s),
            Token::HexLiteral(hex) => write!(f, "{}", hex),
            Token::NumberLiteral(base, exp) if exp.is_empty() => write!(f, "{}", base),
            Token::NumberLiteral(base, exp) => write!(f, "{}e{}", base, exp),
            Token::HexNumber(n) => write!(f, "{}", n),

            Token::DocComment(CommentType::Line, s) => write!(f, "///{}", s),
            Token::DocComment(CommentType::Block, s) => write!(f, "/**{}\n*/", s),

            Token::Package => write!(f, "package"),
            Token::Import => write!(f, "import"),
            Token::Struct => write!(f, "struct"),
            Token::Default => write!(f, "default"),
            Token::As => write!(f, "as"),
            Token::Fun => write!(f, "fun"),

            Token::If => write!(f, "if"),
            Token::Else => write!(f, "else"),
            Token::While => write!(f, "while"),
            Token::For => write!(f, "for"),
            Token::In => write!(f, "in"),
            Token::Range => write!(f, ".."),

            Token::Break => write!(f, "break"),
            Token::Continue => write!(f, "continue"),
            Token::Return => write!(f, "return"),
            Token::Let => write!(f, "let"),

            // type
            Token::Bool => write!(f, "bool"),
            Token::String => write!(f, "string"),
            Token::Uint(w) => write!(f, "uint{}", w),
            Token::Int(w) => write!(f, "int{}", w),
            Token::Bytes(w) => write!(f, "bytes{}", w),
            Token::DynamicBytes => write!(f, "bytes"),

            Token::Binding => write!(f, "$"),
            Token::NewLine => write!(f, "NEWLINE"),

            Token::Semicolon => write!(f, ";"),
            Token::Comma => write!(f, ","),
            Token::OpenParenthesis => write!(f, "("),
            Token::CloseParenthesis => write!(f, ")"),
            Token::OpenCurlyBrace => write!(f, "{{"),
            Token::CloseCurlyBrace => write!(f, "}}"),
            Token::BitwiseXor => write!(f, "^"),

            Token::Or => write!(f, "||"),
            Token::BitwiseOr => write!(f, "|"),
            Token::BitwiseOrAssign => write!(f, "|="),

            Token::And => write!(f, "&&"),
            Token::BitwiseAnd => write!(f, "&"),
            Token::BitwiseAndAssign => write!(f, "&="),

            Token::Add => write!(f, "+"),
            Token::AddAssign => write!(f, "+="),
            Token::Increment => write!(f, "++"),

            Token::Subtract => write!(f, "-"),
            Token::SubtractAssign => write!(f, "-="),
            Token::Decrement => write!(f, "--"),

            Token::Power => write!(f, "**"),
            Token::Mul => write!(f, "*"),
            Token::MulAssign => write!(f, "*="),

            Token::Divide => write!(f, "/"),
            Token::DivideAssign => write!(f, "/="),

            Token::Modulo => write!(f, "%"),
            Token::Equal => write!(f, "=="),
            Token::Assign => write!(f, "="),
            Token::NotEqual => write!(f, "!="),
            Token::Not => write!(f, "!"),

            Token::ShiftLeft => write!(f, "<<"),
            Token::ShiftLeftAssign => write!(f, "<<="),

            Token::ShiftRight => write!(f, "<<"),
            Token::ShiftRightAssign => write!(f, "<<="),

            Token::BitwiseXorAssign => write!(f, "^="),
            Token::ModuloAssign => write!(f, "%="),

            Token::More => write!(f, ">"),
            Token::MoreEqual => write!(f, ">="),
            Token::Member => write!(f, "."),
            Token::Colon => write!(f, ":"),
            Token::OpenBracket => write!(f, "["),
            Token::CloseBracket => write!(f, "]"),
            Token::Complement => write!(f, "~"),
            Token::Question => write!(f, "?"),
            Token::Less => write!(f, "<"),
            Token::LessEqual => write!(f, "<="),
            Token::Arrow => write!(f, "=>"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CommentType {
    Line,
    Block,
}
