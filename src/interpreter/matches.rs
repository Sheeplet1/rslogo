use std::collections::HashMap;

use crate::{
    errors::ExecutionError,
    parser::ast::{Expression, Query},
};

use super::turtle::Turtle;

/// Helper function to match queries to turtle's state.
///
/// Primarily used in the `execute` function to reduce duplicated code.
fn match_queries(query: &Query, turtle: &Turtle) -> f32 {
    match query {
        Query::XCor => turtle.x,
        Query::YCor => turtle.y,
        Query::Heading => turtle.heading as f32,
        Query::Color => turtle.pen_color as f32,
    }
}

/// Helper function to match expressions to their values. This defaults for
/// f32 values. We return an ExecutionError if the expression is not parsable
/// as a float. This is because boolean values are handled elsewhere.
pub fn match_expressions(
    expr: &Expression,
    variables: &HashMap<String, Expression>,
    turtle: &Turtle,
) -> Result<f32, ExecutionError> {
    match expr {
        Expression::Float(val) => Ok(*val),
        Expression::Number(val) => Ok(*val as f32),
        Expression::Usize(val) => Ok(*val as f32),
        Expression::Query(query) => Ok(match_queries(query, turtle)),
        Expression::Variable(var) => get_f32_value(var, variables),
        Expression::Math(expr) => todo!(),
    }
}

/// Helper function to get the value of a variable. Defaults to f32.
fn get_f32_value(
    var: &str,
    variables: &HashMap<String, Expression>,
) -> Result<f32, ExecutionError> {
    if let Some(Expression::Float(val)) = variables.get(var) {
        Ok(*val)
    } else if let Some(Expression::Number(val)) = variables.get(var) {
        Ok(*val as f32)
    } else if let Some(Expression::Usize(val)) = variables.get(var) {
        Ok(*val as f32)
    } else {
        Err(ExecutionError {
            msg: format!(
                "Variable {} does not exist. Consider constructing the variable with MAKE first.",
                var
            ),
        })
    }
}
