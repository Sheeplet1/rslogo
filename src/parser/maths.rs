use std::collections::HashMap;

use crate::errors::ParseError;

use super::ast::Expression;

pub fn parse_maths(
    tokens: &[&str],
    curr_pos: &mut usize,
    variables: &mut HashMap<String, Expression>,
) -> Result<f32, ParseError> {
    // Maths will usually be in the form of: <operator> <expression> <expression>
    // operators will be +, -, *, /

    todo!()
}
