//! Error types for the parser.

#[derive(Debug)]
pub enum ParseErrorKind {
    UnexpectedToken { token: String },
    InvalidSyntax { msg: String },
    VariableNotFound { var: String },
}

#[derive(Debug)]
pub struct ParseError {
    pub kind: ParseErrorKind,
}

impl std::error::Error for ParseError {}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.kind {
            ParseErrorKind::UnexpectedToken { token } => {
                write!(f, "Unexpected token: '{}'", token)
            }
            ParseErrorKind::InvalidSyntax { msg } => {
                write!(f, "Invalid syntax: '{}'.", msg)
            }
            ParseErrorKind::VariableNotFound { var } => {
                write!(f, "Variable not found: '{}'.", var)
            }
        }
    }
}
