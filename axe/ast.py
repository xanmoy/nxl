from dataclasses import dataclass
from typing import Any


class ASTNode:
    pass


# ============================================================
# Program
# ============================================================

@dataclass
class Program(ASTNode):
    statements: list


# ============================================================
# Statements
# ============================================================

@dataclass
class ExpressionStatement(ASTNode):
    expression: Any


@dataclass
class VariableDeclaration(ASTNode):
    name: str
    initializer: Any
    constant: bool = False


@dataclass
class BlockStatement(ASTNode):
    statements: list


@dataclass
class IfStatement(ASTNode):
    condition: Any
    then_branch: Any
    else_branch: Any = None


@dataclass
class ReturnStatement(ASTNode):
    value: Any = None


@dataclass
class FunctionDeclaration(ASTNode):
    name: str
    parameters: list
    body: BlockStatement


# ============================================================
# Expressions
# ============================================================

@dataclass
class BinaryExpression(ASTNode):
    left: Any
    operator: str
    right: Any


@dataclass
class UnaryExpression(ASTNode):
    operator: Any
    operand: Any


@dataclass
class Literal(ASTNode):
    value: Any


@dataclass
class Identifier(ASTNode):
    name: str


@dataclass
class Assignment(ASTNode):
    name: str
    value: Any


@dataclass
class CallExpression(ASTNode):
    callee: Any
    arguments: list