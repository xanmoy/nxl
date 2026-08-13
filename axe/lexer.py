from .tokens import Token, TokenType, KEYWORDS
from .errors import LexerError


class Lexer:
    def __init__(self, source: str):
        self.source = source

        self.start = 0
        self.current = 0

        self.line = 1
        self.column = 1

        self.token_line = 1
        self.token_column = 1

        self.tokens = []

    # =========================================================
    # Public API
    # =========================================================

    def tokenize(self):
        while not self.is_at_end():
            self.start = self.current

            self.token_line = self.line
            self.token_column = self.column

            self.scan_token()

        self.tokens.append(
            Token(
                TokenType.EOF,
                "",
                None,
                self.line,
                self.column,
            )
        )

        return self.tokens

    # =========================================================
    # Character handling
    # =========================================================

    def is_at_end(self):
        return self.current >= len(self.source)

    def advance(self):
        char = self.source[self.current]
        self.current += 1

        if char == "\n":
            self.line += 1
            self.column = 1
        else:
            self.column += 1

        return char

    def peek(self):
        if self.is_at_end():
            return "\0"

        return self.source[self.current]

    def peek_next(self):
        if self.current + 1 >= len(self.source):
            return "\0"

        return self.source[self.current + 1]

    def match(self, expected):
        if self.is_at_end():
            return False

        if self.source[self.current] != expected:
            return False

        self.current += 1
        self.column += 1

        return True

    # =========================================================
    # Token creation
    # =========================================================

    def add_token(self, token_type, literal=None):
        text = self.source[self.start:self.current]

        self.tokens.append(
            Token(
                token_type,
                text,
                literal,
                self.token_line,
                self.token_column,
            )
        )

    # =========================================================
    # Scanner
    # =========================================================

    def scan_token(self):
        char = self.advance()

        # -----------------------------------------------------
        # Whitespace
        # -----------------------------------------------------

        if char in " \t\r":
            return

        # -----------------------------------------------------
        # Newline
        # -----------------------------------------------------

        if char == "\n":
            self.add_token(TokenType.NEWLINE)
            return

        # -----------------------------------------------------
        # Comments
        # -----------------------------------------------------

        if char == "/" and self.peek() == "/":
            self.skip_comment()
            return

        # -----------------------------------------------------
        # Single-character tokens
        # -----------------------------------------------------

        single_chars = {
            "(": TokenType.LPAREN,
            ")": TokenType.RPAREN,
            "{": TokenType.LBRACE,
            "}": TokenType.RBRACE,
            "[": TokenType.LBRACKET,
            "]": TokenType.RBRACKET,
            ",": TokenType.COMMA,
            ".": TokenType.DOT,
            ":": TokenType.COLON,
            "+": TokenType.PLUS,
            "-": TokenType.MINUS,
            "*": TokenType.STAR,
            "%": TokenType.PERCENT,
        }

        if char in single_chars:

            # ->
            if char == "-" and self.peek() == ">":
                self.advance()
                self.add_token(TokenType.ARROW)

            else:
                self.add_token(single_chars[char])

            return

        # -----------------------------------------------------
        # Division
        # -----------------------------------------------------

        if char == "/":
            self.add_token(TokenType.SLASH)
            return

        # -----------------------------------------------------
        # Assignment / equality
        # -----------------------------------------------------

        if char == "=":
            if self.match("="):
                self.add_token(TokenType.EQUAL_EQUAL)
            else:
                self.add_token(TokenType.EQUAL)

            return

        # -----------------------------------------------------
        # Not equal
        # -----------------------------------------------------

        if char == "!":

            if self.match("="):
                self.add_token(TokenType.NOT_EQUAL)
                return

            raise LexerError(
                "Unexpected character '!'",
                self.token_line,
                self.token_column,
            )

        # -----------------------------------------------------
        # Greater than
        # -----------------------------------------------------

        if char == ">":

            if self.match("="):
                self.add_token(TokenType.GREATER_EQUAL)
            else:
                self.add_token(TokenType.GREATER)

            return

        # -----------------------------------------------------
        # Less than
        # -----------------------------------------------------

        if char == "<":

            if self.match("="):
                self.add_token(TokenType.LESS_EQUAL)
            else:
                self.add_token(TokenType.LESS)

            return

        # -----------------------------------------------------
        # Strings
        # -----------------------------------------------------

        if char == '"':
            self.string('"')
            return

        if char == "'":
            self.string("'")
            return

        # -----------------------------------------------------
        # Numbers
        # -----------------------------------------------------

        if char.isdigit():
            self.number()
            return

        # -----------------------------------------------------
        # Identifiers / keywords
        # -----------------------------------------------------

        if self.is_identifier_start(char):
            self.identifier()
            return

        # -----------------------------------------------------
        # Unknown character
        # -----------------------------------------------------

        raise LexerError(
            f"Unexpected character {char!r}",
            self.token_line,
            self.token_column,
        )

    # =========================================================
    # Comments
    # =========================================================

    def skip_comment(self):
        while self.peek() not in ("\n", "\0"):
            self.advance()

    # =========================================================
    # Strings
    # =========================================================

    def string(self, quote):
        value = []

        while not self.is_at_end() and self.peek() != quote:

            char = self.advance()

            if char == "\\":

                if self.is_at_end():
                    raise LexerError(
                        "Unterminated escape sequence",
                        self.token_line,
                        self.token_column,
                    )

                escaped = self.advance()

                escapes = {
                    "n": "\n",
                    "t": "\t",
                    "r": "\r",
                    "\\": "\\",
                    '"': '"',
                    "'": "'",
                }

                value.append(
                    escapes.get(escaped, escaped)
                )

            else:
                value.append(char)

        if self.is_at_end():
            raise LexerError(
                "Unterminated string",
                self.token_line,
                self.token_column,
            )

        # Closing quote
        self.advance()

        self.add_token(
            TokenType.STRING,
            "".join(value),
        )

    # =========================================================
    # Numbers
    # =========================================================

    def number(self):

        while self.peek().isdigit():
            self.advance()

        # Decimal number
        if (
            self.peek() == "."
            and self.peek_next().isdigit()
        ):
            self.advance()

            while self.peek().isdigit():
                self.advance()

        text = self.source[
            self.start:self.current
        ]

        if "." in text:
            value = float(text)
        else:
            value = int(text)

        self.add_token(
            TokenType.NUMBER,
            value,
        )

    # =========================================================
    # Identifiers / keywords
    # =========================================================

    def identifier(self):

        while self.is_identifier_part(
            self.peek()
        ):
            self.advance()

        text = self.source[
            self.start:self.current
        ]

        token_type = KEYWORDS.get(
            text,
            TokenType.IDENTIFIER,
        )

        literal = None

        if token_type == TokenType.TRUE:
            literal = True

        elif token_type == TokenType.FALSE:
            literal = False

        elif token_type == TokenType.NULL:
            literal = None

        self.add_token(
            token_type,
            literal,
        )

    # =========================================================
    # Identifier rules
    # =========================================================

    @staticmethod
    def is_identifier_start(char):
        return (
            char.isalpha()
            or char == "_"
        )

    @staticmethod
    def is_identifier_part(char):
        return (
            char.isalnum()
            or char == "_"
        )