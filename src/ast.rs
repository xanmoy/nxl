use crate::token::TokenType;

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    VariableDeclaration {
        name: String,
        initializer: Expression,
        constant: bool,
    },

    Expression(Expression),

    Block {
        statements: Vec<Statement>,
    },

    If {
        condition: Expression,
        then_branch: Box<Statement>,
        else_branch: Option<Box<Statement>>,
    },

    Function {
        name: String,
        parameters: Vec<String>,
        body: Box<Statement>,
    },

    Return(Option<Expression>),
}

#[derive(Debug)]
pub enum Expression {
    Literal(LiteralValue),

    Identifier(String),

    Binary {
        left: Box<Expression>,
        operator: TokenType,
        right: Box<Expression>,
    },

    Unary {
        operator: TokenType,
        operand: Box<Expression>,
    },

    Assignment {
        name: String,
        value: Box<Expression>,
    },

    Call {
        callee: Box<Expression>,
        arguments: Vec<Expression>,
    },
}

#[derive(Debug)]
pub enum LiteralValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
}