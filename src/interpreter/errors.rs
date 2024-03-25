#[derive(Debug)]
pub enum ExecutionErrorKind {
    DivisionByZero,
    VariableNotFound { var: String },
    TypeError { expected: String },
}

#[derive(Debug)]
pub struct ExecutionError {
    pub kind: ExecutionErrorKind,
}

impl std::error::Error for ExecutionError {}

impl std::fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.kind {
            ExecutionErrorKind::DivisionByZero => {
                write!(f, "Division by zero")
            }
            ExecutionErrorKind::VariableNotFound { var } => {
                write!(f, "Variable not found: '{}'", var)
            }
            ExecutionErrorKind::TypeError { expected } => {
                write!(f, "Type error: expected '{}'", expected)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let error = ExecutionError {
            kind: ExecutionErrorKind::DivisionByZero,
        };
        assert_eq!(error.to_string(), "Division by zero");

        let error = ExecutionError {
            kind: ExecutionErrorKind::VariableNotFound {
                var: "x".to_string(),
            },
        };
        assert_eq!(error.to_string(), "Variable not found: 'x'");

        let error = ExecutionError {
            kind: ExecutionErrorKind::TypeError {
                expected: "number".to_string(),
            },
        };
        assert_eq!(error.to_string(), "Type error: expected 'number'");
    }
}
