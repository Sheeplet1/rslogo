//! # Control Flows
//!
//! Contains the implementation for the execution of control flow statements.

use std::collections::HashMap;

use crate::{
    errors::ExecutionError,
    parser::ast::{ASTNode, Condition, Expression},
};

use super::{execute::execute, matches::match_expressions, turtle::Turtle};

/// Generic comparator function to compare two expressions.
fn comparator(
    lhs: &Expression,
    rhs: &Expression,
    comparator: fn(f32, f32) -> bool,
    turtle: &Turtle,
    variables: &HashMap<String, Expression>,
) -> Result<bool, ExecutionError> {
    let lhs_val = match_expressions(lhs, variables, turtle)?;
    let rhs_val = match_expressions(rhs, variables, turtle)?;
    Ok(comparator(lhs_val, rhs_val))
}

/// Function to evaluate and execute an `if` statement.
///
/// # Example
///
/// ```rust
/// use std::collections::HashMap;
/// use turtle::interpreter::control_flows::eval_exec_if;
/// use turtle::interpreter::errors::ExecutionError;
/// use turtle::parser::ast::{ASTNode, Condition, Expression};
/// use turtle::interpreter::turtle::Turtle;
///
/// let condition = Condition::Equals(
///    Box::new(Expression::Number(1.0)),
///    Box::new(Expression::Number(1.0)),
///    );
/// let block = vec![ASTNode::Forward(Expression::Number(10.0))];
/// let mut turtle = Turtle::new();
/// let mut variables = HashMap::new();
/// let result = eval_exec_if(&condition, &block, &mut turtle, &mut variables);
/// assert_eq!(result, Ok(()));
/// ```
pub fn eval_exec_if(
    condition: &Condition,
    block: &Vec<ASTNode>,
    turtle: &mut Turtle,
    variables: &mut HashMap<String, Expression>,
) -> Result<(), ExecutionError> {
    let exec = should_execute(condition, turtle, variables)?;

    if exec {
        execute(block, turtle, variables)?;
    }

    Ok(())
}

/// Function to evaluate and execute a `while` statement.
///
/// # Example
///
/// ```rust
/// use std::collections::HashMap;
/// use turtle::interpreter::control_flows::eval_exec_while;
/// use turtle::interpreter::errors::ExecutionError;
/// use turtle::parser::ast::{ASTNode, Condition, Expression};
/// use turtle::interpreter::turtle::Turtle;
///
/// let condition = Condition::Equals(
///   Box::new(Expression::Number(1.0)),
///   Box::new(Expression::Number(1.0)),
///   );
/// let block = vec![ASTNode::Forward(Expression::Number(10.0))];
/// let mut turtle = Turtle::new();
/// let mut variables = HashMap::new();
/// let result = eval_exec_while(&condition, &block, &mut turtle, &mut variables);
/// assert_eq!(result, Ok(()));
/// ```
pub fn eval_exec_while(
    condition: &Condition,
    block: &Vec<ASTNode>,
    turtle: &mut Turtle,
    variables: &mut HashMap<String, Expression>,
) -> Result<(), ExecutionError> {
    let mut exec = should_execute(condition, turtle, variables)?;

    while exec {
        execute(block, turtle, variables)?;

        exec = should_execute(condition, turtle, variables)?;
    }

    Ok(())
}

/// Evaluates a condition and returns a boolean to determine if a conditional
/// block should be executed.
fn should_execute(
    condition: &Condition,
    turtle: &Turtle,
    variables: &HashMap<String, Expression>,
) -> Result<bool, ExecutionError> {
    match condition {
        Condition::Equals(lhs, rhs) => comparator(lhs, rhs, |a, b| a == b, turtle, variables),
        Condition::LessThan(lhs, rhs) => comparator(lhs, rhs, |a, b| a < b, turtle, variables),
        Condition::GreaterThan(lhs, rhs) => comparator(lhs, rhs, |a, b| a > b, turtle, variables),
        Condition::And(lhs, rhs) => {
            comparator(lhs, rhs, |a, b| a != 0.0 && b != 0.0, turtle, variables)
        }
        Condition::Or(lhs, rhs) => {
            comparator(lhs, rhs, |a, b| a != 0.0 || b != 0.0, turtle, variables)
        }
    }
}
