#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Literals
    Identifier,
    Number,
    String,

    // Keywords
    Let,
    Const,
    Fn,
    If,
    Else,
    For,
    While,
    Return,

    True,
    False,
    Null,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,

    Equal,
    EqualEqual,
    NotEqual,

    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    And,
    Or,
    Not,

    // Punctuation
    LeftParen,
    RightParen,

    LeftBrace,
    RightBrace,

    LeftBracket,
    RightBracket,

    Comma,
    Dot,
    Colon,

    Arrow,

    // Special
    Newline,
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: impl Into<String>,
        literal: Option<Literal>,
        line: usize,
        column: usize,
    ) -> Self {
        Self {
            token_type,
            lexeme: lexeme.into(),
            literal,
            line,
            column,
        }
    }
}