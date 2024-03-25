//! Contains helper functions to match expressions to their values.
//! Defaults to a f32 value and returns an ExecutionError if
//! the expression is not parsable as a float.

use std::collections::HashMap;

use crate::parser::ast::{Expression, Math, Query};

use super::{
    errors::{ExecutionError, ExecutionErrorKind},
    turtle::Turtle,
};

/// Helper function to match queries to turtle's state.
///
/// # Example
///
/// ```rust
/// let mut image = Image::new(100, 100);
/// let turtle = Turtle::new(&mut image);
///
/// let res = match_queries(&Query::XCor, &turtle);
/// assert_eq!(res, 50.0);
/// ```
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
///
/// let res = match_expressions(&expr, &HashMap::new(), &Turtle::new()).unwrap();
/// assert_eq!(res, 1.0);
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
///
/// let image = Image::new(100, 100);
/// let turtle = Turtle::new(&mut image);
///
/// let res = get_var_val("x", &variables, &turtle).unwrap();
/// assert_eq!(res, 1.0);
/// ```
fn get_var_val(
    var: &str,
    variables: &HashMap<String, Expression>,
    turtle: &Turtle,
) -> Result<f32, ExecutionError> {
    // TODO: Hate this, refactor.
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
            kind: ExecutionErrorKind::VariableNotFound {
                var: var.to_string(),
            },
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
/// let res = eval_binary_op(&lhs, &rhs, &HashMap::new(), &Turtle::new(), |a, b| a + b).unwrap();
/// assert_eq!(res, 3.0);
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
/// let expr = Math::Add(Expression::Float(1.0), Expression::Float(2.0));
///
/// let res = eval_math(&expr, &HashMap::new(), &Turtle::new()).unwrap();
/// assert_eq!(res, 3.0);
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
                    kind: ExecutionErrorKind::DivisionByZero,
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
    use unsvg::Image;

    use super::*;
    use crate::parser::ast::Query;

    #[test]
    fn test_match_queries() {
        let mut image = Image::new(100, 100);
        let turtle = Turtle::new(&mut image);

        let res = match_queries(&Query::XCor, &turtle);
        assert_eq!(res, 50.0);

        let res = match_queries(&Query::YCor, &turtle);
        assert_eq!(res, 50.0);

        let res = match_queries(&Query::Heading, &turtle);
        assert_eq!(res, 0.0);

        let res = match_queries(&Query::Color, &turtle);
        assert_eq!(res, 7.0);
    }

    #[test]
    fn test_match_expressions() {
        let mut variables = HashMap::new();
        variables.insert("x".to_string(), Expression::Float(1.0));
        let mut image = Image::new(100, 100);
        let turtle = Turtle::new(&mut image);

        let res = match_expressions(&Expression::Float(1.0), &variables, &turtle).unwrap();
        assert_eq!(res, 1.0);

        let res = match_expressions(&Expression::Number(1), &variables, &turtle).unwrap();
        assert_eq!(res, 1.0);

        let res = match_expressions(&Expression::Usize(1), &variables, &turtle).unwrap();
        assert_eq!(res, 1.0);

        let res = match_expressions(&Expression::Query(Query::XCor), &variables, &turtle).unwrap();
        assert_eq!(res, 50.0);

        let res =
            match_expressions(&Expression::Variable("x".to_string()), &variables, &turtle).unwrap();
        assert_eq!(res, 1.0);

        let res = match_expressions(
            &Expression::Math(Box::new(Math::Add(
                Expression::Float(1.0),
                Expression::Float(2.0),
            ))),
            &variables,
            &turtle,
        )
        .unwrap();
        assert_eq!(res, 3.0);
    }

    #[test]
    fn test_get_var_val() {
        let mut variables = HashMap::new();

        variables.insert("float".to_string(), Expression::Float(1.0));
        variables.insert("number".to_string(), Expression::Number(1));
        variables.insert("usize".to_string(), Expression::Usize(1));
        variables.insert("query".to_string(), Expression::Query(Query::XCor));
        variables.insert(
            "math".to_string(),
            Expression::Math(Box::new(Math::Add(
                Expression::Float(1.0),
                Expression::Float(2.0),
            ))),
        );

        let mut image = Image::new(100, 100);
        let turtle = Turtle::new(&mut image);

        let res = get_var_val("float", &variables, &turtle).unwrap();
        assert_eq!(res, 1.0);

        let res = get_var_val("number", &variables, &turtle).unwrap();
        assert_eq!(res, 1.0);

        let res = get_var_val("usize", &variables, &turtle).unwrap();
        assert_eq!(res, 1.0);

        let res = get_var_val("query", &variables, &turtle).unwrap();
        assert_eq!(res, 50.0);

        let res = get_var_val("math", &variables, &turtle).unwrap();
        assert_eq!(res, 3.0);
    }

    #[test]
    fn test_get_var_val_error() {
        let variables = HashMap::new();
        let mut image = Image::new(100, 100);
        let turtle = Turtle::new(&mut image);

        let res = get_var_val("x", &variables, &turtle);
        assert!(res.is_err());
    }

    #[test]
    fn test_eval_binary_op() {
        let variables = HashMap::new();
        let mut image = Image::new(100, 100);
        let turtle = Turtle::new(&mut image);

        let lhs = Expression::Float(1.0);
        let rhs = Expression::Float(2.0);

        let res = eval_binary_op(&lhs, &rhs, &variables, &turtle, |a, b| a + b).unwrap();
        assert_eq!(res, 3.0);
    }

    #[test]
    fn test_eval_logical_op() {
        let variables = HashMap::new();
        let mut image = Image::new(100, 100);
        let turtle = Turtle::new(&mut image);

        let lhs = Expression::Float(1.0);
        let rhs = Expression::Float(2.0);

        let res = eval_logical_op(&lhs, &rhs, &variables, &turtle, |a, b| {
            if a < b {
                1.0
            } else {
                0.0
            }
        })
        .unwrap();
        assert_eq!(res, 1.0);

        let res = eval_logical_op(&lhs, &rhs, &variables, &turtle, |a, b| {
            if a > b {
                1.0
            } else {
                0.0
            }
        })
        .unwrap();
        assert_eq!(res, 0.0);
    }

    #[test]
    fn test_eval_math_add() {
        let variables = HashMap::new();
        let mut image = Image::new(100, 100);
        let turtle = Turtle::new(&mut image);

        let expr = Math::Add(Expression::Float(1.0), Expression::Float(2.0));

        let res = eval_math(&expr, &variables, &turtle).unwrap();
        assert_eq!(res, 3.0);
    }

    #[test]
    fn test_eval_math_sub() {
        let variables = HashMap::new();
        let mut image = Image::new(100, 100);
        let turtle = Turtle::new(&mut image);

        let expr = Math::Sub(Expression::Float(1.0), Expression::Float(2.0));

        let res = eval_math(&expr, &variables, &turtle).unwrap();
        assert_eq!(res, -1.0);
    }

    #[test]
    fn test_eval_math_mul() {
        let variables = HashMap::new();
        let mut image = Image::new(100, 100);
        let turtle = Turtle::new(&mut image);

        let expr = Math::Mul(Expression::Float(1.0), Expression::Float(2.0));

        let res = eval_math(&expr, &variables, &turtle).unwrap();
        assert_eq!(res, 2.0);
    }

    #[test]
    fn test_eval_math_div() {
        let variables = HashMap::new();
        let mut image = Image::new(100, 100);
        let turtle = Turtle::new(&mut image);

        let expr = Math::Div(Expression::Float(1.0), Expression::Float(2.0));

        let res = eval_math(&expr, &variables, &turtle).unwrap();
        assert_eq!(res, 0.5);
    }

    #[test]
    fn test_eval_math_div_by_zero() {
        let variables = HashMap::new();
        let mut image = Image::new(100, 100);
        let turtle = Turtle::new(&mut image);

        let expr = Math::Div(Expression::Float(1.0), Expression::Float(0.0));

        let res = eval_math(&expr, &variables, &turtle);
        assert!(res.is_err());
    }

    #[test]
    fn test_eval_math_eq() {
        let variables = HashMap::new();
        let mut image = Image::new(100, 100);
        let turtle = Turtle::new(&mut image);

        let expr = Math::Eq(Expression::Float(1.0), Expression::Float(1.0));

        let res = eval_math(&expr, &variables, &turtle).unwrap();
        assert_eq!(res, 1.0);
    }

    #[test]
    fn test_eval_math_lt() {
        let variables = HashMap::new();
        let mut image = Image::new(100, 100);
        let turtle = Turtle::new(&mut image);

        let expr = Math::Lt(Expression::Float(1.0), Expression::Float(2.0));

        let res = eval_math(&expr, &variables, &turtle).unwrap();
        assert_eq!(res, 1.0);
    }

    #[test]
    fn test_eval_math_gt() {
        let variables = HashMap::new();
        let mut image = Image::new(100, 100);
        let turtle = Turtle::new(&mut image);

        let expr = Math::Gt(Expression::Float(1.0), Expression::Float(2.0));

        let res = eval_math(&expr, &variables, &turtle).unwrap();
        assert_eq!(res, 0.0);
    }

    #[test]
    fn test_eval_math_ne() {
        let variables = HashMap::new();
        let mut image = Image::new(100, 100);
        let turtle = Turtle::new(&mut image);

        let expr = Math::Ne(Expression::Float(1.0), Expression::Float(2.0));

        let res = eval_math(&expr, &variables, &turtle).unwrap();
        assert_eq!(res, 1.0);
    }

    #[test]
    fn test_eval_math_and() {
        let variables = HashMap::new();
        let mut image = Image::new(100, 100);
        let turtle = Turtle::new(&mut image);

        let expr = Math::And(Expression::Float(1.0), Expression::Float(2.0));

        let res = eval_math(&expr, &variables, &turtle).unwrap();
        assert_eq!(res, 1.0);
    }

    #[test]
    fn test_eval_math_or() {
        let variables = HashMap::new();
        let mut image = Image::new(100, 100);
        let turtle = Turtle::new(&mut image);

        let expr = Math::Or(Expression::Float(1.0), Expression::Float(0.0));

        let res = eval_math(&expr, &variables, &turtle).unwrap();
        assert_eq!(res, 1.0);
    }

    #[test]
    fn test_eval_math_or_false() {
        let variables = HashMap::new();
        let mut image = Image::new(100, 100);
        let turtle = Turtle::new(&mut image);

        let expr = Math::Or(Expression::Float(0.0), Expression::Float(0.0));

        let res = eval_math(&expr, &variables, &turtle).unwrap();
        assert_eq!(res, 0.0);
    }
}
