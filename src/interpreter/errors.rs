#[derive(Debug)]
pub enum ExecutionErrorKind {
    DivisionByZero,
    VariableNotFound { var_name: String },
    TypeError { expected: String },
}

#[derive(Debug)]
pub struct ExecutionError {
    pub kind: ExecutionErrorKind,
    pub msg: Option<String>,
}

impl ExecutionError {
    pub fn div_by_zero() -> Self {
        ExecutionError {
            kind: ExecutionErrorKind::DivisionByZero,
            msg: Some("Attempted division by zero.".to_string()),
        }
    }

    pub fn var_not_found(var_name: &str) -> Self {
        ExecutionError {
            kind: ExecutionErrorKind::VariableNotFound {
                var_name: var_name.into(),
            },
            msg: Some(format!("Variable '{}' not found.", var_name)),
        }
    }

    pub fn type_error(expected: &str) -> Self {
        ExecutionError {
            kind: ExecutionErrorKind::TypeError {
                expected: expected.into(),
            },
            msg: Some(format!("Expected type '{}'", expected)),
        }
    }
}

impl std::error::Error for ExecutionError {}

impl std::fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let _ = match &self.kind {
            ExecutionErrorKind::DivisionByZero => write!(f, "Division by zero error."),
            ExecutionErrorKind::VariableNotFound { var_name } => {
                write!(f, "Variable not found: {}", var_name)
            }
            ExecutionErrorKind::TypeError { expected } => {
                write!(f, "Expected type '{}'.", expected)
            }
        };

        if let Some(msg) = &self.msg {
            write!(f, " {}", msg)
        } else {
            Ok(())
        }
    }
}
