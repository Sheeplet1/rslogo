//! Contains helper functions to match expressions to their values.
//! Defaults to a f32 value and returns an ExecutionError if
//! the expression is not parsable as a float.

use std::collections::HashMap;

use crate::{
    errors::ExecutionError,
    parser::ast::{Expression, Math, Query},
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
/// as a float.
///
/// # Example
///
/// ```rust
/// let expr = Expression::Float(1.0);
/// let res = match_expressions(&expr, &HashMap::new(), &Turtle::new());
/// assert_eq!(res, Ok(1.0));
/// ```
pub fn match_expressions(
    expr: &Expression,
    variables: &HashMap<String, Expression>,
    turtle: &Turtle,
) -> Result<f32, ExecutionError> {
    match expr {
        Expression::Float(val) => Ok(*val),
        // NOTE: What is the point of this is we are just casting it to f32?
        Expression::Number(val) => Ok(*val as f32),
        Expression::Usize(val) => Ok(*val as f32),
        Expression::Query(query) => Ok(match_queries(query, turtle)),
        Expression::Variable(var) => get_var_val(var, variables, turtle),
        Expression::Math(expr) => Ok(eval_math(expr, variables, turtle)?),
    }
}

/// Gets the value of a variable from the variables hashmap.
///
/// # Example
///
/// ```rust
/// let mut variables = HashMap::new();
/// variables.insert("x".to_string(), Expression::Float(1.0));
/// let turtle = Turtle::new();
/// let res = get_var_val("x", &variables, &turtle);
/// assert_eq!(res, Ok(1.0));
/// ```
fn get_var_val(
    var: &str,
    variables: &HashMap<String, Expression>,
    turtle: &Turtle,
) -> Result<f32, ExecutionError> {
    if let Some(Expression::Float(val)) = variables.get(var) {
        Ok(*val)
    } else if let Some(Expression::Number(val)) = variables.get(var) {
        Ok(*val as f32)
    } else if let Some(Expression::Usize(val)) = variables.get(var) {
        Ok(*val as f32)
    } else if let Some(Expression::Query(query)) = variables.get(var) {
        Ok(match_queries(query, turtle))
    } else if let Some(Expression::Math(expr)) = variables.get(var) {
        Ok(eval_math(expr, variables, turtle)?)
    } else {
        Err(ExecutionError {
            msg: format!(
                "Variable {} does not exist. Consider constructing the variable with MAKE first.",
                var
            ),
        })
    }
}

/// Evaluates a binary operation and returns the result.
///
/// # Example
///
/// ```rust
/// let lhs = Expression::Float(1.0);
/// let rhs = Expression::Float(2.0);
///
/// let res = eval_binary_op(&lhs, &rhs, &HashMap::new(), &Turtle::new(), |a, b| a + b);
/// assert_eq!(res, Ok(3.0));
/// ```
fn eval_binary_op(
    lhs: &Expression,
    rhs: &Expression,
    variables: &HashMap<String, Expression>,
    turtle: &Turtle,
    op: fn(f32, f32) -> f32,
) -> Result<f32, ExecutionError> {
    let lhs_val = match_expressions(lhs, variables, turtle)?;
    let rhs_val = match_expressions(rhs, variables, turtle)?;
    Ok(op(lhs_val, rhs_val))
}

/// Evaluates a logical operation and returns the result.
///
/// # Example
///
/// ```rust
/// let lhs = Expression::Float(1.0);
/// let rhs = Expression::Float(2.0);
///
/// let res = eval_logical_op(&lhs, &rhs, &HashMap::new(), &Turtle::new(), |a, b| a + b);
/// assert_eq!(res, Ok(1.0));
/// ```
fn eval_logical_op(
    lhs: &Expression,
    rhs: &Expression,
    variables: &HashMap<String, Expression>,
    turtle: &Turtle,
    op: fn(f32, f32) -> f32,
) -> Result<f32, ExecutionError> {
    let lhs_val = match_expressions(lhs, variables, turtle)?;
    let rhs_val = match_expressions(rhs, variables, turtle)?;
    if op(lhs_val, rhs_val) != 0.0 {
        Ok(1.0)
    } else {
        Ok(0.0)
    }
}

/// Evaluates a Math expression and returns the result. Math expressions are
/// basic arithmetics or logical operations.
///
/// # Example
///
/// ```rust
/// let expr = Math::Add(Box::new(Expression::Float(1.0)), Box::new(Expression::Float(2.0)));
///
/// let res = eval_math(&expr, &HashMap::new(), &Turtle::new());
/// assert_eq!(res, Ok(3.0));
/// ```
fn eval_math(
    expr: &Math,
    variables: &HashMap<String, Expression>,
    turtle: &Turtle,
) -> Result<f32, ExecutionError> {
    match expr {
        Math::Add(lhs, rhs) => eval_binary_op(lhs, rhs, variables, turtle, |a, b| a + b),
        Math::Sub(lhs, rhs) => eval_binary_op(lhs, rhs, variables, turtle, |a, b| a - b),
        Math::Mul(lhs, rhs) => eval_binary_op(lhs, rhs, variables, turtle, |a, b| a * b),
        Math::Div(lhs, rhs) => {
            let rhs_val = match_expressions(rhs, variables, turtle)?;
            if rhs_val == 0.0 {
                return Err(ExecutionError {
                    msg: "Division by zero".to_string(),
                });
            }
            Ok(eval_binary_op(lhs, rhs, variables, turtle, |a, b| a / b)?)
        }
        Math::Eq(lhs, rhs) => {
            eval_logical_op(
                lhs,
                rhs,
                variables,
                turtle,
                |a, b| if a == b { 1.0 } else { 0.0 },
            )
        }
        Math::Lt(lhs, rhs) => {
            eval_logical_op(
                lhs,
                rhs,
                variables,
                turtle,
                |a, b| if a < b { 1.0 } else { 0.0 },
            )
        }
        Math::Gt(lhs, rhs) => {
            eval_logical_op(
                lhs,
                rhs,
                variables,
                turtle,
                |a, b| if a > b { 1.0 } else { 0.0 },
            )
        }
        Math::Ne(lhs, rhs) => {
            eval_logical_op(
                lhs,
                rhs,
                variables,
                turtle,
                |a, b| if a != b { 1.0 } else { 0.0 },
            )
        }
        Math::And(lhs, rhs) => eval_logical_op(lhs, rhs, variables, turtle, |a, b| a * b),
        Math::Or(lhs, rhs) => eval_logical_op(lhs, rhs, variables, turtle, |a, b| {
            if a + b > 0.0 {
                1.0
            } else {
                0.0
            }
        }),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use unsvg::Image;

    use super::*;
    use crate::parser::ast::Expression;

    #[test]
    fn test_get_var_val() {
        let mut variables = HashMap::new();
        variables.insert("x".to_string(), Expression::Float(1.0));
        let mut image = Image::new(100, 100);
        let turtle = Turtle {
            x: (100 / 2) as f32,
            y: (100 / 2) as f32,
            heading: 0,
            pen_down: false,
            pen_color: 7, // White
            image: &mut image,
        };
        let res = get_var_val("x", &variables, &turtle).unwrap();
        assert_eq!(res, 1.0);
    }
}
