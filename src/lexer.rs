use crate::error::NxlError;
use crate::token::{Literal, Token, TokenType};

pub struct Lexer {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
    column: usize,
    token_line: usize,
    token_column: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
            column: 1,
            token_line: 1,
            token_column: 1,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, NxlError> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            self.start = self.current;
            self.token_line = self.line;
            self.token_column = self.column;

            self.scan_token(&mut tokens)?;
        }

        tokens.push(Token::new(
            TokenType::Eof,
            "",
            None,
            self.line,
            self.column,
        ));

        Ok(tokens)
    }

    fn scan_token(&mut self, tokens: &mut Vec<Token>) -> Result<(), NxlError> {
        let c = self.advance();

        match c {
            // Whitespace
            ' ' | '\t' | '\r' => {}

            // Newline
            '\n' => {
                self.add_token(tokens, TokenType::Newline, None);
            }

            // Single-character tokens
            '(' => self.add_token(tokens, TokenType::LeftParen, None),
            ')' => self.add_token(tokens, TokenType::RightParen, None),

            '{' => self.add_token(tokens, TokenType::LeftBrace, None),
            '}' => self.add_token(tokens, TokenType::RightBrace, None),

            '[' => self.add_token(tokens, TokenType::LeftBracket, None),
            ']' => self.add_token(tokens, TokenType::RightBracket, None),

            ',' => self.add_token(tokens, TokenType::Comma, None),
            '.' => self.add_token(tokens, TokenType::Dot, None),
            ':' => self.add_token(tokens, TokenType::Colon, None),

            '+' => self.add_token(tokens, TokenType::Plus, None),
            '*' => self.add_token(tokens, TokenType::Star, None),
            '%' => self.add_token(tokens, TokenType::Percent, None),

            // Minus or arrow
            '-' => {
                if self.match_char('>') {
                    self.add_token(tokens, TokenType::Arrow, None);
                } else {
                    self.add_token(tokens, TokenType::Minus, None);
                }
            }

            // Division or comment
            '/' => {
                if self.match_char('/') {
                    self.skip_comment();
                } else {
                    self.add_token(tokens, TokenType::Slash, None);
                }
            }

            // Assignment / equality
            '=' => {
                if self.match_char('=') {
                    self.add_token(tokens, TokenType::EqualEqual, None);
                } else {
                    self.add_token(tokens, TokenType::Equal, None);
                }
            }

            // Not equal
            '!' => {
                if self.match_char('=') {
                    self.add_token(tokens, TokenType::NotEqual, None);
                } else {
                    return Err(self.error("Unexpected character '!'"));
                }
            }

            // Greater / greater equal
            '>' => {
                if self.match_char('=') {
                    self.add_token(tokens, TokenType::GreaterEqual, None);
                } else {
                    self.add_token(tokens, TokenType::Greater, None);
                }
            }

            // Less / less equal
            '<' => {
                if self.match_char('=') {
                    self.add_token(tokens, TokenType::LessEqual, None);
                } else {
                    self.add_token(tokens, TokenType::Less, None);
                }
            }

            // Strings
            '"' => {
                self.string('"', tokens)?;
            }

            '\'' => {
                self.string('\'', tokens)?;
            }

            // Numbers
            c if c.is_ascii_digit() => {
                self.number(tokens);
            }

            // Identifiers / keywords
            c if Self::is_identifier_start(c) => {
                self.identifier(tokens);
            }

            // Unknown character
            c => {
                return Err(self.error(&format!(
                    "Unexpected character '{}'",
                    c
                )));
            }
        }

        Ok(())
    }

    // =========================================================
    // Strings
    // =========================================================

    fn string(
        &mut self,
        quote: char,
        tokens: &mut Vec<Token>,
    ) -> Result<(), NxlError> {
        let mut value = String::new();

        while !self.is_at_end() && self.peek() != quote {
            let c = self.advance();

            if c == '\\' {
                if self.is_at_end() {
                    return Err(self.error("Unterminated escape sequence"));
                }

                let escaped = self.advance();

                match escaped {
                    'n' => value.push('\n'),
                    't' => value.push('\t'),
                    'r' => value.push('\r'),
                    '\\' => value.push('\\'),
                    '"' => value.push('"'),
                    '\'' => value.push('\''),
                    other => value.push(other),
                }
            } else {
                value.push(c);
            }
        }

        if self.is_at_end() {
            return Err(self.error("Unterminated string"));
        }

        // Closing quote
        self.advance();

        self.add_token(
            tokens,
            TokenType::String,
            Some(Literal::String(value)),
        );

        Ok(())
    }

    // =========================================================
    // Numbers
    // =========================================================

    fn number(&mut self, tokens: &mut Vec<Token>) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let text = self.lexeme();

        let value = text.parse::<f64>().unwrap();

        self.add_token(
            tokens,
            TokenType::Number,
            Some(Literal::Number(value)),
        );
    }

    // =========================================================
    // Identifiers / Keywords
    // =========================================================

    fn identifier(&mut self, tokens: &mut Vec<Token>) {
        while Self::is_identifier_part(self.peek()) {
            self.advance();
        }

        let text = self.lexeme();

        let token_type = match text.as_str() {
            "let" => TokenType::Let,
            "const" => TokenType::Const,
            "fn" => TokenType::Fn,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "for" => TokenType::For,
            "while" => TokenType::While,
            "return" => TokenType::Return,

            "true" => TokenType::True,
            "false" => TokenType::False,
            "null" => TokenType::Null,

            "and" => TokenType::And,
            "or" => TokenType::Or,
            "not" => TokenType::Not,

            _ => TokenType::Identifier,
        };

        let literal = match token_type {
            TokenType::True => Some(Literal::Boolean(true)),
            TokenType::False => Some(Literal::Boolean(false)),
            TokenType::Null => Some(Literal::Null),
            _ => None,
        };

        self.add_token(tokens, token_type, literal);
    }

    // =========================================================
    // Comments
    // =========================================================

    fn skip_comment(&mut self) {
        while !self.is_at_end() && self.peek() != '\n' {
            self.advance();
        }
    }

    // =========================================================
    // Character handling
    // =========================================================

    fn advance(&mut self) -> char {
        let c = self.source[self.current];

        self.current += 1;

        if c == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }

        c
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source[self.current] != expected {
            return false;
        }

        self.current += 1;
        self.column += 1;

        true
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    // =========================================================
    // Token creation
    // =========================================================

    fn add_token(
        &self,
        tokens: &mut Vec<Token>,
        token_type: TokenType,
        literal: Option<Literal>,
    ) {
        tokens.push(Token::new(
            token_type,
            self.lexeme(),
            literal,
            self.token_line,
            self.token_column,
        ));
    }

    fn lexeme(&self) -> String {
        self.source[self.start..self.current]
            .iter()
            .collect()
    }

    // =========================================================
    // Identifier rules
    // =========================================================

    fn is_identifier_start(c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }

    fn is_identifier_part(c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '_'
    }

    // =========================================================
    // Errors
    // =========================================================

    fn error(&self, message: &str) -> NxlError {
        NxlError::new(
            message,
            self.token_line,
            self.token_column,
        )
    }
}