#[derive(Debug)]
pub enum ParseErrorKind {
    UnexpectedToken { expected: String, found: String },
    InvalidSyntax { details: String },
    UnsupportedFeature { feat: String },
    VariableNotFound { var: String },
}

#[derive(Debug)]
pub struct ParseError {
    pub kind: ParseErrorKind,
    pub line: Option<usize>,
    pub col: Option<usize>,
}

impl ParseError {
    pub fn new(kind: ParseErrorKind, line: Option<usize>, col: Option<usize>) -> Self {
        Self { kind, line, col }
    }
}

impl std::error::Error for ParseError {}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.kind {
            ParseErrorKind::UnexpectedToken { expected, found } => {
                write!(
                    f,
                    "Unexpected token. Expected: '{}', found: '{}'.",
                    expected, found
                );
            }
            ParseErrorKind::InvalidSyntax { details } => {
                write!(f, "Invalid syntax: '{}'.", details);
            }
            ParseErrorKind::UnsupportedFeature { feat } => {
                write!(f, "Unsupported feature: '{}'.", feat);
            }
            ParseErrorKind::VariableNotFound { var } => {
                write!(f, "Variable not found: '{}'.", var);
            }
        }

        if let (Some(line), Some(col)) = (self.line, self.col) {
            write!(f, " at line {}, column {}.", line, col)
        } else {
            Ok(())
        }
    }
}
