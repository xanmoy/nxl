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

    // =========================================================
    // Program execution
    // =========================================================

    pub fn interpret(&mut self, program: &Program) -> Result<(), NxlError> {
        for statement in &program.statements {
            self.execute(statement)?;
        }

        Ok(())
    }

    // =========================================================
    // Statements
    // =========================================================

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

    // =========================================================
    // Expressions
    // =========================================================

    fn evaluate(
        &mut self,
        expression: &Expression,
    ) -> Result<Value, NxlError> {
        match expression {
            // Literal values
            Expression::Literal(value) => {
                Ok(self.literal_to_value(value))
            }

            // Variables
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

            // Function calls
            Expression::Call {
                callee,
                arguments,
            } => {
                self.evaluate_call(callee, arguments)
            }

            _ => {
                Err(self.error(
                    "Expression is not supported by the interpreter yet.",
                ))
            }
        }
    }

    // =========================================================
    // Function calls
    // =========================================================

    fn evaluate_call(
        &mut self,
        callee: &Expression,
        arguments: &[Expression],
    ) -> Result<Value, NxlError> {
        let function_name = match callee {
            Expression::Identifier(name) => name,

            _ => {
                return Err(self.error(
                    "Can only call functions by name.",
                ));
            }
        };

        match function_name.as_str() {
            "print" => {
                self.builtin_print(arguments)
            }

            _ => {
                Err(self.error(&format!(
                    "Undefined function '{}'.",
                    function_name
                )))
            }
        }
    }

    // =========================================================
    // Built-in: print
    // =========================================================

    fn builtin_print(
        &mut self,
        arguments: &[Expression],
    ) -> Result<Value, NxlError> {
        if arguments.len() != 1 {
            return Err(self.error(
                "print() expects exactly one argument.",
            ));
        }

        let value = self.evaluate(&arguments[0])?;

        println!("{}", self.value_to_string(&value));

        Ok(Value::Null)
    }

    // =========================================================
    // Literal conversion
    // =========================================================

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

    // =========================================================
    // Runtime value → output string
    // =========================================================

    fn value_to_string(&self, value: &Value) -> String {
        match value {
            Value::Number(number) => {
                if number.fract() == 0.0 {
                    format!("{}", *number as i64)
                } else {
                    number.to_string()
                }
            }

            Value::String(string) => {
                string.clone()
            }

            Value::Boolean(boolean) => {
                boolean.to_string()
            }

            Value::Null => {
                "null".to_string()
            }
        }
    }

    // =========================================================
    // Runtime errors
    // =========================================================

    fn error(&self, message: &str) -> NxlError {
        NxlError::new(message, 0, 0)
    }
}