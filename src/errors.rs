//! # Errors
//!
//! This module contains the error types used in the library.
//! The error types are used to represent the different types of errors that can occur during the execution of the library.
//!
//! The error types are:
//! - `ParseError`: Represents an error that occurs during parsing.
//! - `ExtendedUnsvgError`: Represents an error that occurs during usage of the unsvg library.
//! - `ExecutionError`: Represents an error that occurs during the execution of the library.

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
