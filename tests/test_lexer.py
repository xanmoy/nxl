from axe.lexer import Lexer
from axe.tokens import TokenType


def test_basic_tokens():
    source = """
    let x = 10
    """

    tokens = Lexer(source).tokenize()

    assert tokens[0].type == TokenType.NEWLINE
    assert tokens[1].type == TokenType.LET
    assert tokens[2].type == TokenType.IDENTIFIER
    assert tokens[3].type == TokenType.EQUAL
    assert tokens[4].type == TokenType.NUMBER
    assert tokens[4].literal == 10