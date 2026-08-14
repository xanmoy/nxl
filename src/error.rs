use std::fmt;

#[derive(Debug)]
pub struct NxlError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl NxlError {
    pub fn new(
        message: impl Into<String>,
        line: usize,
        column: usize,
    ) -> Self {
        Self {
            message: message.into(),
            line,
            column,
        }
    }
}

impl fmt::Display for NxlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[line {}, column {}] {}",
            self.line,
            self.column,
            self.message
        )
    }
}

impl std::error::Error for NxlError {}