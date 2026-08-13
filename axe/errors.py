class AXEError(Exception):
    """Base class for all AXE errors."""

    def __init__(self, message: str, line: int = 0, column: int = 0):
        self.message = message
        self.line = line
        self.column = column

        if line > 0:
            location = f"[line {line}"

            if column > 0:
                location += f", column {column}"

            location += "] "

            message = location + message

        super().__init__(message)


class LexerError(AXEError):
    """Raised when AXE source cannot be tokenized."""

    pass


class ParserError(AXEError):
    """Raised when AXE tokens cannot be parsed."""

    pass