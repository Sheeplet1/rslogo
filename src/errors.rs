//! # Errors
//!
//! This module contains the error types used in the library.
//! The error types are used to represent the different types of errors that can occur during the execution of the library.
//!
//! The error types are:
//! - `ParseError`: Represents an error that occurs during parsing.
//! - `ExtendedUnsvgError`: Represents an error that occurs during usage of the unsvg library.
//! - `ExecutionError`: Represents an error that occurs during the execution of the library.

#[derive(PartialEq)]
pub struct ParseError {
    pub msg: String,
    // pub line: usize,
    // pub col: usize,
}

impl std::error::Error for ParseError {}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Parse error: {}", self.msg)
    }
}

impl std::fmt::Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Parse error: {}", self.msg)
    }
}

#[derive(Debug)]
pub struct ExtendedUnsvgError {
    pub msg: String,
}

impl std::error::Error for ExtendedUnsvgError {}

impl std::fmt::Display for ExtendedUnsvgError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, ":9 {}", self.msg)
    }
}

#[derive(Debug)]
pub struct ExecutionError {
    pub msg: String,
}

impl std::error::Error for ExecutionError {}

impl std::fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Execution error: {}", self.msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_error() {
        let parse_error = ParseError {
            msg: "Parse error".to_string(),
        };
        assert_eq!(format!("{}", parse_error), "Parse error: Parse error");
    }

    #[test]
    fn test_parse_debug() {
        let parse_error = ParseError {
            msg: "Parse error".to_string(),
        };
        assert_eq!(format!("{:?}", parse_error), "Parse error: Parse error")
    }

    #[test]
    fn test_extended_unsvg_error() {
        let extended_unsvg_error = ExtendedUnsvgError {
            msg: "Extended unsvg error".to_string(),
        };
        assert_eq!(
            format!("{}", extended_unsvg_error),
            ":9 Extended unsvg error"
        );
    }

    #[test]
    fn test_execution_error() {
        let execution_error = ExecutionError {
            msg: "Execution error".to_string(),
        };
        assert_eq!(
            format!("{}", execution_error),
            "Execution error: Execution error"
        );
    }
}
