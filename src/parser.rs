use crate::ast::{Expression, LiteralValue, Program, Statement};

use crate::error::NxlError;
use crate::token::{Literal, Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Program, NxlError> {
        let mut statements = Vec::new();

        self.skip_newlines();

        while !self.is_at_end() {
            statements.push(self.statement()?);
            self.skip_newlines();
        }

        Ok(Program { statements })
    }

    // ========================================================
    // Statements
    // ========================================================

    fn statement(&mut self) -> Result<Statement, NxlError> {
        if self.match_token(TokenType::Let) {
            return self.variable_declaration(false);
        }

        if self.match_token(TokenType::Const) {
            return self.variable_declaration(true);
        }

        if self.match_token(TokenType::Fn) {
            return self.function_declaration();
        }

        if self.match_token(TokenType::If) {
            return self.if_statement();
        }

        if self.match_token(TokenType::Return) {
            return self.return_statement();
        }

        Ok(Statement::Expression(self.expression()?))
    }

    fn variable_declaration(&mut self, constant: bool) -> Result<Statement, NxlError> {
        let name = self.consume(TokenType::Identifier, "Expected variable name.")?;

        self.consume(TokenType::Equal, "Expected '=' after variable name.")?;

        let initializer = self.expression()?;

        Ok(Statement::VariableDeclaration {
            name: name.lexeme,
            initializer,
            constant,
        })
    }

    // ========================================================
    // Blocks
    // ========================================================

    fn block(&mut self) -> Result<Statement, NxlError> {
        self.consume(TokenType::LeftBrace, "Expected '{'.")?;

        self.skip_newlines();

        let mut statements = Vec::new();

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.statement()?);
            self.skip_newlines();
        }

        self.consume(TokenType::RightBrace, "Expected '}'.")?;

        Ok(Statement::Block { statements })
    }

    // ========================================================
    // If
    // ========================================================

    fn if_statement(&mut self) -> Result<Statement, NxlError> {
        let condition = self.expression()?;

        let then_branch = Box::new(self.block()?);

        let else_branch = if self.match_token(TokenType::Else) {
            Some(Box::new(self.block()?))
        } else {
            None
        };

        Ok(Statement::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    // ========================================================
    // Functions
    // ========================================================

    fn function_declaration(&mut self) -> Result<Statement, NxlError> {
        let name = self.consume(TokenType::Identifier, "Expected function name.")?;

        self.consume(TokenType::LeftParen, "Expected '(' after function name.")?;

        let mut parameters = Vec::new();

        if !self.check(TokenType::RightParen) {
            loop {
                let parameter = self.consume(TokenType::Identifier, "Expected parameter name.")?;

                parameters.push(parameter.lexeme);

                if !self.match_token(TokenType::Comma) {
                    break;
                }
            }
        }

        self.consume(TokenType::RightParen, "Expected ')' after parameters.")?;

        let body = Box::new(self.block()?);

        Ok(Statement::Function {
            name: name.lexeme,
            parameters,
            body,
        })
    }

    // ========================================================
    // Return
    // ========================================================

    fn return_statement(&mut self) -> Result<Statement, NxlError> {
        if self.check(TokenType::Newline) || self.check(TokenType::RightBrace) {
            return Ok(Statement::Return(None));
        }

        let value = self.expression()?;

        Ok(Statement::Return(Some(value)))
    }

    // ========================================================
    // Expressions
    // ========================================================

    fn expression(&mut self) -> Result<Expression, NxlError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expression, NxlError> {
        let expression = self.logical_or()?;

        if self.match_token(TokenType::Equal) {
            let value = self.assignment()?;

            if let Expression::Identifier(name) = expression {
                return Ok(Expression::Assignment {
                    name,
                    value: Box::new(value),
                });
            }

            return Err(self.error("Invalid assignment target."));
        }

        Ok(expression)
    }

    // ========================================================
    // Logical OR
    // ========================================================

    fn logical_or(&mut self) -> Result<Expression, NxlError> {
        let mut expression = self.logical_and()?;

        while self.match_token(TokenType::Or) {
            let operator = self.previous().token_type.clone();
            let right = self.logical_and()?;

            expression = Expression::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expression)
    }

    // ========================================================
    // Logical AND
    // ========================================================

    fn logical_and(&mut self) -> Result<Expression, NxlError> {
        let mut expression = self.equality()?;

        while self.match_token(TokenType::And) {
            let operator = self.previous().token_type.clone();
            let right = self.equality()?;

            expression = Expression::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expression)
    }

    // ========================================================
    // Equality
    // ========================================================

    fn equality(&mut self) -> Result<Expression, NxlError> {
        let mut expression = self.comparison()?;

        while self.match_any(&[TokenType::EqualEqual, TokenType::NotEqual]) {
            let operator = self.previous().token_type.clone();
            let right = self.comparison()?;

            expression = Expression::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expression)
    }

    // ========================================================
    // Comparison
    // ========================================================

    fn comparison(&mut self) -> Result<Expression, NxlError> {
        let mut expression = self.term()?;

        while self.match_any(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().token_type.clone();
            let right = self.term()?;

            expression = Expression::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expression)
    }

    // ========================================================
    // Addition / subtraction
    // ========================================================

    fn term(&mut self) -> Result<Expression, NxlError> {
        let mut expression = self.factor()?;

        while self.match_any(&[TokenType::Plus, TokenType::Minus]) {
            let operator = self.previous().token_type.clone();
            let right = self.factor()?;

            expression = Expression::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expression)
    }

    // ========================================================
    // Multiplication
    // ========================================================

    fn factor(&mut self) -> Result<Expression, NxlError> {
        let mut expression = self.unary()?;

        while self.match_any(&[TokenType::Star, TokenType::Slash, TokenType::Percent]) {
            let operator = self.previous().token_type.clone();
            let right = self.unary()?;

            expression = Expression::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expression)
    }

    // ========================================================
    // Unary
    // ========================================================

    fn unary(&mut self) -> Result<Expression, NxlError> {
        if self.match_any(&[TokenType::Not, TokenType::Minus]) {
            let operator = self.previous().token_type.clone();
            let operand = self.unary()?;

            return Ok(Expression::Unary {
                operator,
                operand: Box::new(operand),
            });
        }

        self.call()
    }

    // ========================================================
    // Function calls
    // ========================================================

    fn call(&mut self) -> Result<Expression, NxlError> {
        let mut expression = self.primary()?;

        loop {
            if self.match_token(TokenType::LeftParen) {
                expression = self.finish_call(expression)?;
            } else {
                break;
            }
        }

        Ok(expression)
    }

    fn finish_call(&mut self, callee: Expression) -> Result<Expression, NxlError> {
        let mut arguments = Vec::new();

        if !self.check(TokenType::RightParen) {
            loop {
                arguments.push(self.expression()?);

                if !self.match_token(TokenType::Comma) {
                    break;
                }
            }
        }

        self.consume(TokenType::RightParen, "Expected ')' after arguments.")?;

        Ok(Expression::Call {
            callee: Box::new(callee),
            arguments,
        })
    }

    // ========================================================
    // Primary
    // ========================================================

    fn primary(&mut self) -> Result<Expression, NxlError> {
        if self.match_token(TokenType::False) {
            return Ok(Expression::Literal(LiteralValue::Boolean(false)));
        }

        if self.match_token(TokenType::True) {
            return Ok(Expression::Literal(LiteralValue::Boolean(true)));
        }

        if self.match_token(TokenType::Null) {
            return Ok(Expression::Literal(LiteralValue::Null));
        }

        if self.match_token(TokenType::Number) {
            if let Some(Literal::Number(value)) = self.previous().literal.clone() {
                return Ok(Expression::Literal(LiteralValue::Number(value)));
            }
        }

        if self.match_token(TokenType::String) {
            if let Some(Literal::String(value)) = self.previous().literal.clone() {
                return Ok(Expression::Literal(LiteralValue::String(value)));
            }
        }

        if self.match_token(TokenType::Identifier) {
            return Ok(Expression::Identifier(self.previous().lexeme.clone()));
        }

        if self.match_token(TokenType::LeftParen) {
            let expression = self.expression()?;

            self.consume(TokenType::RightParen, "Expected ')' after expression.")?;

            return Ok(expression);
        }

        Err(self.error("Unexpected token."))
    }

    // ========================================================
    // Helpers
    // ========================================================

    fn match_token(&mut self, token_type: TokenType) -> bool {
        if self.check(token_type) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn match_any(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(token_type.clone()) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, NxlError> {
        if self.check(token_type) {
            return Ok(self.advance());
        }

        Err(self.error(message))
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return token_type == TokenType::Eof;
        }

        self.peek().token_type == token_type
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous().clone()
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn skip_newlines(&mut self) {
        while self.match_token(TokenType::Newline) {}
    }

    fn error(&self, message: &str) -> NxlError {
        let token = self.peek();

        NxlError::new(message, token.line, token.column)
    }
}
