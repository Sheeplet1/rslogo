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
/// as a float. This is because boolean values are handled elsewhere.
pub fn match_expressions(
    expr: &Expression,
    vars: &HashMap<String, Expression>,
    turtle: &Turtle,
) -> Result<f32, ExecutionError> {
    match expr {
        Expression::Float(val) => Ok(*val),
        // TODO: What is the point of this is we are just casting it to f32?
        Expression::Number(val) => Ok(*val as f32),
        Expression::Usize(val) => Ok(*val as f32),
        Expression::Query(query) => Ok(match_queries(query, turtle)),
        Expression::Variable(var) => get_var_val(var, vars, turtle),
        Expression::Math(expr) => Ok(eval_math(expr, vars, turtle)?),
    }
}

/// Helper function to get the value of a variable. Defaults to f32.
fn get_var_val(
    var: &str,
    vars: &HashMap<String, Expression>,
    turtle: &Turtle,
) -> Result<f32, ExecutionError> {
    // TODO: Refactor
    if let Some(Expression::Float(val)) = vars.get(var) {
        Ok(*val)
    } else if let Some(Expression::Number(val)) = vars.get(var) {
        Ok(*val as f32)
    } else if let Some(Expression::Usize(val)) = vars.get(var) {
        Ok(*val as f32)
    } else if let Some(Expression::Query(query)) = vars.get(var) {
        Ok(match_queries(query, turtle))
    } else if let Some(Expression::Math(expr)) = vars.get(var) {
        Ok(eval_math(expr, vars, turtle)?)
    } else {
        Err(ExecutionError {
            msg: format!(
                "Variable {} does not exist. Consider constructing the variable with MAKE first.",
                var
            ),
        })
    }
}

fn eval_binary_op<T>(
    lhs: &Expression,
    rhs: &Expression,
    vars: &HashMap<String, Expression>,
    turtle: &Turtle,
    op: T,
) -> Result<f32, ExecutionError>
where
    T: Fn(f32, f32) -> f32,
{
    let lhs_val = match_expressions(lhs, vars, turtle)?;
    let rhs_val = match_expressions(rhs, vars, turtle)?;
    Ok(op(lhs_val, rhs_val))
}

fn eval_math(
    expr: &Math,
    vars: &HashMap<String, Expression>,
    turtle: &Turtle,
) -> Result<f32, ExecutionError> {
    match expr {
        Math::Add(lhs, rhs) => Ok(eval_binary_op(lhs, rhs, vars, turtle, |a, b| a + b)?),
        Math::Sub(lhs, rhs) => Ok(eval_binary_op(lhs, rhs, vars, turtle, |a, b| a - b)?),
        Math::Mul(lhs, rhs) => Ok(eval_binary_op(lhs, rhs, vars, turtle, |a, b| a * b)?),
        Math::Div(lhs, rhs) => {
            let rhs_val = match_expressions(rhs, vars, turtle)?;
            if rhs_val == 0.0 {
                return Err(ExecutionError {
                    msg: "Division by zero".to_string(),
                });
            }
            Ok(eval_binary_op(lhs, rhs, vars, turtle, |a, b| a / b)?)
        }
        Math::Eq(lhs, rhs) => Ok(eval_binary_op(lhs, rhs, vars, turtle, |a, b| {
            if a == b {
                1.0
            } else {
                0.0
            }
        })?),
        Math::Lt(lhs, rhs) => Ok(eval_binary_op(lhs, rhs, vars, turtle, |a, b| {
            if a < b {
                1.0
            } else {
                0.0
            }
        })?),
        Math::Gt(lhs, rhs) => Ok(eval_binary_op(lhs, rhs, vars, turtle, |a, b| {
            if a > b {
                1.0
            } else {
                0.0
            }
        })?),
        Math::Ne(lhs, rhs) => Ok(eval_binary_op(lhs, rhs, vars, turtle, |a, b| {
            if a != b {
                1.0
            } else {
                0.0
            }
        })?),
        Math::And(lhs, rhs) => Ok(eval_binary_op(lhs, rhs, vars, turtle, |a, b| {
            if a != 0.0 && b != 0.0 {
                1.0
            } else {
                0.0
            }
        })?),
        Math::Or(lhs, rhs) => Ok(eval_binary_op(lhs, rhs, vars, turtle, |a, b| {
            if a != 0.0 || b != 0.0 {
                1.0
            } else {
                0.0
            }
        })?),
    }
}
