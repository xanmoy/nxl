from axe.lexer import Lexer
from axe.parser import Parser
from axe.ast import Program, VariableDeclaration


def parse(source):
    tokens = Lexer(source).tokenize()
    return Parser(tokens).parse()


def test_variable_declaration():
    program = parse("""
    let x = 10
    """)

    assert isinstance(program, Program)
    assert len(program.statements) == 1

    statement = program.statements[0]

    assert isinstance(statement, VariableDeclaration)
    assert statement.name == "x"
    assert statement.initializer.value == 10