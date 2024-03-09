use std::{error::Error, fmt::Display};

pub struct ParseError {
    pub msg: String,
}

impl Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParseError: {}", self.msg)
    }
}
