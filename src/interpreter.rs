use crate::ast::{Expression, LiteralValue, Program, Statement};
use crate::environment::Environment;
use crate::error::NxlError;
use crate::runtime::Value;

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    pub fn interpret(&mut self, program: &Program) -> Result<(), NxlError> {
        for statement in &program.statements {
            self.execute(statement)?;
        }

        Ok(())
    }

    fn execute(&mut self, statement: &Statement) -> Result<(), NxlError> {
        match statement {
            Statement::VariableDeclaration {
                name,
                initializer,
                ..
            } => {
                let value = self.evaluate(initializer)?;

                self.environment.define(
                    name.clone(),
                    value,
                );

                Ok(())
            }

            Statement::Expression(expression) => {
                self.evaluate(expression)?;
                Ok(())
            }

            Statement::Block { statements } => {
                for statement in statements {
                    self.execute(statement)?;
                }

                Ok(())
            }

            _ => {
                Err(self.error(
                    "Statement is not supported by the interpreter yet.",
                ))
            }
        }
    }

    fn evaluate(
        &mut self,
        expression: &Expression,
    ) -> Result<Value, NxlError> {
        match expression {
            Expression::Literal(value) => {
                Ok(self.literal_to_value(value))
            }

            Expression::Identifier(name) => {
                self.environment
                    .get(name)
                    .ok_or_else(|| {
                        self.error(&format!(
                            "Undefined variable '{}'.",
                            name
                        ))
                    })
            }

            _ => {
                Err(self.error(
                    "Expression is not supported by the interpreter yet.",
                ))
            }
        }
    }

    fn literal_to_value(
        &self,
        literal: &LiteralValue,
    ) -> Value {
        match literal {
            LiteralValue::Number(value) => {
                Value::Number(*value)
            }

            LiteralValue::String(value) => {
                Value::String(value.clone())
            }

            LiteralValue::Boolean(value) => {
                Value::Boolean(*value)
            }

            LiteralValue::Null => {
                Value::Null
            }
        }
    }

    fn error(&self, message: &str) -> NxlError {
        NxlError::new(message, 0, 0)
    }
}