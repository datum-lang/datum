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

    Rarrow,

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
        use Token::*;
        match self {
            Identifier(id) => write!(f, "{}", id),
            StringLiteral(s) => write!(f, "\"{}\"", s),
            HexLiteral(hex) => write!(f, "{}", hex),
            NumberLiteral(base, exp) if exp.is_empty() => write!(f, "{}", base),
            NumberLiteral(base, exp) => write!(f, "{}e{}", base, exp),
            HexNumber(n) => write!(f, "{}", n),

            DocComment(CommentType::Line, s) => write!(f, "///{}", s),
            DocComment(CommentType::Block, s) => write!(f, "/**{}\n*/", s),

            Package => write!(f, "package"),
            Import => write!(f, "import"),
            Struct => write!(f, "struct"),
            Default => write!(f, "default"),
            As => write!(f, "as"),
            Fun => write!(f, "fun"),

            If => write!(f, "if"),
            Else => write!(f, "else"),
            While => write!(f, "while"),
            For => write!(f, "for"),
            In => write!(f, "in"),
            Range => write!(f, ".."),

            Break => write!(f, "break"),
            Continue => write!(f, "continue"),
            Return => write!(f, "return"),
            Let => write!(f, "let"),

            // type
            Bool => write!(f, "bool"),
            String => write!(f, "string"),
            Uint(w) => write!(f, "uint{}", w),
            Int(w) => write!(f, "int{}", w),
            Bytes(w) => write!(f, "bytes{}", w),
            DynamicBytes => write!(f, "bytes"),

            Binding => write!(f, "$"),
            NewLine => write!(f, "NEWLINE"),

            Semicolon => write!(f, ";"),
            Comma => write!(f, ","),
            OpenParenthesis => write!(f, "("),
            CloseParenthesis => write!(f, ")"),
            OpenCurlyBrace => write!(f, "{{"),
            CloseCurlyBrace => write!(f, "}}"),
            BitwiseXor => write!(f, "^"),

            Or => write!(f, "||"),
            BitwiseOr => write!(f, "|"),
            BitwiseOrAssign => write!(f, "|="),

            And => write!(f, "&&"),
            BitwiseAnd => write!(f, "&"),
            BitwiseAndAssign => write!(f, "&="),

            Add => write!(f, "+"),
            AddAssign => write!(f, "+="),
            Increment => write!(f, "++"),

            Subtract => write!(f, "-"),
            SubtractAssign => write!(f, "-="),
            Decrement => write!(f, "--"),

            Power => write!(f, "**"),
            Mul => write!(f, "*"),
            MulAssign => write!(f, "*="),

            Divide => write!(f, "/"),
            DivideAssign => write!(f, "/="),

            Modulo => write!(f, "%"),
            Equal => write!(f, "=="),
            Assign => write!(f, "="),
            NotEqual => write!(f, "!="),
            Not => write!(f, "!"),

            ShiftLeft => write!(f, "<<"),
            ShiftLeftAssign => write!(f, "<<="),

            ShiftRight => write!(f, "<<"),
            ShiftRightAssign => write!(f, "<<="),

            BitwiseXorAssign => write!(f, "^="),
            ModuloAssign => write!(f, "%="),

            More => write!(f, ">"),
            MoreEqual => write!(f, ">="),
            Member => write!(f, "."),
            Colon => write!(f, ":"),
            OpenBracket => write!(f, "["),
            CloseBracket => write!(f, "]"),
            Complement => write!(f, "~"),
            Question => write!(f, "?"),
            Less => write!(f, "<"),
            LessEqual => write!(f, "<="),
            Arrow => write!(f, "=>"),
            Rarrow => f.write_str("'->'"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CommentType {
    Line,
    Block,
}
