from .tokens import TokenType
from .errors import ParserError
from .ast import (
    Program,
    ExpressionStatement,
    VariableDeclaration,
    BlockStatement,
    IfStatement,
    ReturnStatement,
    FunctionDeclaration,
    BinaryExpression,
    UnaryExpression,
    Literal,
    Identifier,
    Assignment,
    CallExpression,
)


class Parser:
    def __init__(self, tokens):
        self.tokens = tokens
        self.current = 0

    # ========================================================
    # Entry point
    # ========================================================

    def parse(self):
        statements = []

        self.skip_newlines()

        while not self.is_at_end():
            statements.append(self.statement())
            self.skip_newlines()

        return Program(statements)

    # ========================================================
    # Statements
    # ========================================================

    def statement(self):
        if self.match(TokenType.LET):
            return self.variable_declaration(False)

        if self.match(TokenType.CONST):
            return self.variable_declaration(True)

        if self.match(TokenType.IF):
            return self.if_statement()

        if self.match(TokenType.FN):
            return self.function_declaration()

        if self.match(TokenType.RETURN):
            return self.return_statement()

        return self.expression_statement()

    def variable_declaration(self, constant):
        name = self.consume(
            TokenType.IDENTIFIER,
            "Expected variable name.",
        )

        self.consume(
            TokenType.EQUAL,
            "Expected '=' after variable name.",
        )

        initializer = self.expression()

        return VariableDeclaration(
            name=name.lexeme,
            initializer=initializer,
            constant=constant,
        )

    def expression_statement(self):
        expression = self.expression()

        return ExpressionStatement(expression)

    # ========================================================
    # IF
    # ========================================================

    def if_statement(self):
        condition = self.expression()

        then_branch = self.block()

        else_branch = None

        if self.match(TokenType.ELSE):
            if self.match(TokenType.IF):
                else_branch = self.if_statement()
            else:
                else_branch = self.block()

        return IfStatement(
            condition,
            then_branch,
            else_branch,
        )

    # ========================================================
    # Functions
    # ========================================================

    def function_declaration(self):
        name = self.consume(
            TokenType.IDENTIFIER,
            "Expected function name.",
        )

        self.consume(
            TokenType.LPAREN,
            "Expected '(' after function name.",
        )

        parameters = []

        if not self.check(TokenType.RPAREN):
            while True:
                parameter = self.consume(
                    TokenType.IDENTIFIER,
                    "Expected parameter name.",
                )

                parameters.append(parameter.lexeme)

                if not self.match(TokenType.COMMA):
                    break

        self.consume(
            TokenType.RPAREN,
            "Expected ')' after parameters.",
        )

        body = self.block()

        return FunctionDeclaration(
            name=name.lexeme,
            parameters=parameters,
            body=body,
        )

    # ========================================================
    # Return
    # ========================================================

    def return_statement(self):
        if self.check(TokenType.NEWLINE):
            return ReturnStatement()

        if self.check(TokenType.RBRACE):
            return ReturnStatement()

        return ReturnStatement(
            self.expression()
        )

    # ========================================================
    # Blocks
    # ========================================================

    def block(self):
        self.consume(
            TokenType.LBRACE,
            "Expected '{'.",
        )

        self.skip_newlines()

        statements = []

        while (
            not self.check(TokenType.RBRACE)
            and not self.is_at_end()
        ):
            statements.append(self.statement())
            self.skip_newlines()

        self.consume(
            TokenType.RBRACE,
            "Expected '}'.",
        )

        return BlockStatement(statements)

    # ========================================================
    # Expressions
    # ========================================================

    def expression(self):
        return self.assignment()

    def assignment(self):
        expression = self.logical_or()

        if self.match(TokenType.EQUAL):
            equals = self.previous()
            value = self.assignment()

            if isinstance(expression, Identifier):
                return Assignment(
                    expression.name,
                    value,
                )

            raise self.error(
                equals,
                "Invalid assignment target.",
            )

        return expression

    # ========================================================
    # Logical operators
    # ========================================================

    def logical_or(self):
        expression = self.logical_and()

        while self.match(TokenType.OR):
            operator = self.previous().lexeme
            right = self.logical_and()

            expression = BinaryExpression(
                expression,
                operator,
                right,
            )

        return expression

    def logical_and(self):
        expression = self.equality()

        while self.match(TokenType.AND):
            operator = self.previous().lexeme
            right = self.equality()

            expression = BinaryExpression(
                expression,
                operator,
                right,
            )

        return expression

    # ========================================================
    # Equality
    # ========================================================

    def equality(self):
        expression = self.comparison()

        while self.match(
            TokenType.EQUAL_EQUAL,
            TokenType.NOT_EQUAL,
        ):
            operator = self.previous().lexeme
            right = self.comparison()

            expression = BinaryExpression(
                expression,
                operator,
                right,
            )

        return expression

    # ========================================================
    # Comparison
    # ========================================================

    def comparison(self):
        expression = self.term()

        while self.match(
            TokenType.GREATER,
            TokenType.GREATER_EQUAL,
            TokenType.LESS,
            TokenType.LESS_EQUAL,
        ):
            operator = self.previous().lexeme
            right = self.term()

            expression = BinaryExpression(
                expression,
                operator,
                right,
            )

        return expression

    # ========================================================
    # Addition / subtraction
    # ========================================================

    def term(self):
        expression = self.factor()

        while self.match(
            TokenType.PLUS,
            TokenType.MINUS,
        ):
            operator = self.previous().lexeme
            right = self.factor()

            expression = BinaryExpression(
                expression,
                operator,
                right,
            )

        return expression

    # ========================================================
    # Multiplication
    # ========================================================

    def factor(self):
        expression = self.unary()

        while self.match(
            TokenType.STAR,
            TokenType.SLASH,
            TokenType.PERCENT,
        ):
            operator = self.previous().lexeme
            right = self.unary()

            expression = BinaryExpression(
                expression,
                operator,
                right,
            )

        return expression

    # ========================================================
    # Unary
    # ========================================================

    def unary(self):
        if self.match(
            TokenType.NOT,
            TokenType.MINUS,
        ):
            operator = self.previous().lexeme

            return UnaryExpression(
                operator,
                self.unary(),
            )

        return self.call()

    # ========================================================
    # Function calls
    # ========================================================

    def call(self):
        expression = self.primary()

        while True:
            if self.match(TokenType.LPAREN):
                expression = self.finish_call(expression)
            else:
                break

        return expression

    def finish_call(self, callee):
        arguments = []

        if not self.check(TokenType.RPAREN):
            while True:
                arguments.append(self.expression())

                if not self.match(TokenType.COMMA):
                    break

        self.consume(
            TokenType.RPAREN,
            "Expected ')' after arguments.",
        )

        return CallExpression(
            callee,
            arguments,
        )

    # ========================================================
    # Primary
    # ========================================================

    def primary(self):
        if self.match(TokenType.FALSE):
            return Literal(False)

        if self.match(TokenType.TRUE):
            return Literal(True)

        if self.match(TokenType.NULL):
            return Literal(None)

        if self.match(TokenType.NUMBER):
            return Literal(self.previous().literal)

        if self.match(TokenType.STRING):
            return Literal(self.previous().literal)

        if self.match(TokenType.IDENTIFIER):
            return Identifier(self.previous().lexeme)

        if self.match(TokenType.LPAREN):
            expression = self.expression()

            self.consume(
                TokenType.RPAREN,
                "Expected ')' after expression.",
            )

            return expression

        token = self.peek()

        raise self.error(
            token,
            f"Unexpected token '{token.lexeme}'.",
        )

    # ========================================================
    # Helpers
    # ========================================================

    def match(self, *types):
        for token_type in types:
            if self.check(token_type):
                self.advance()
                return True

        return False

    def consume(self, token_type, message):
        if self.check(token_type):
            return self.advance()

        raise self.error(
            self.peek(),
            message,
        )

    def check(self, token_type):
        if self.is_at_end():
            return token_type == TokenType.EOF

        return self.peek().type == token_type

    def advance(self):
        if not self.is_at_end():
            self.current += 1

        return self.previous()

    def peek(self):
        return self.tokens[self.current]

    def previous(self):
        return self.tokens[self.current - 1]

    def is_at_end(self):
        return self.peek().type == TokenType.EOF

    def skip_newlines(self):
        while self.match(TokenType.NEWLINE):
            pass

    def error(self, token, message):
        return ParserError(
            message,
            token.line,
            token.column,
        )