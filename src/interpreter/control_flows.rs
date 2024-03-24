//! Control flow execution functions.
//!
//! Responsible for evaluating conditions and executing the block if the
//! condition is true.

use std::collections::HashMap;

use crate::parser::ast::{ASTNode, Condition, Expression};

use super::{errors::ExecutionError, execute::execute, matches::match_expressions, turtle::Turtle};

/// Compares two expressions using a given comparator.
///
/// # Examples
///
/// ```rust
/// use std::collections::HashMap;
/// use turtle::Turtle;
/// use parser::ast::{Condition, Expression};
/// use interpreter::errors::ExecutionError;
///
/// let mut vars: HashMap<String, Expression> = HashMap::new();
/// let turtle = Turtle::new();
///
/// let lhs = Expression::Float(8.0);
/// let rhs = Expression::Float(10.0);
///
/// let res = comparator(&lhs, &rhs, |a, b| a < b, &turtle, &vars);
/// assert_eq!(res, Ok(true));
/// ```
fn comparator(
    lhs: &Expression,
    rhs: &Expression,
    comparator: fn(f32, f32) -> bool,
    turtle: &Turtle,
    vars: &HashMap<String, Expression>,
) -> Result<bool, ExecutionError> {
    let lhs_val = match_expressions(lhs, vars, turtle)?;
    let rhs_val = match_expressions(rhs, vars, turtle)?;
    Ok(comparator(lhs_val, rhs_val))
}

/// Evaluates the condition and executes an `IF` block if the condition is true.
///
/// # Examples
/// ```rust
/// use std::collections::HashMap;
/// use turtle::Turtle;
/// use parser::ast::{ASTNode, Condition, Expression};
/// use interpreter::control_flows::eval_exec_if;
/// use interpreter::errors::ExecutionError;
///
/// let mut vars: HashMap<String, Expression> = HashMap::new();
/// let mut turtle = Turtle::new();
///
/// let condition = Condition::LessThan(
///   Box::new(Expression::Float(8.0)),
///   Box::new(Expression::Float(10.0)),
/// );
///
/// let block = vec![ASTNode::Command(Command::Forward(Expression::Float(100.0)))];
/// let result = eval_exec_if(&condition, &block, &mut turtle, &mut vars);
/// assert_eq!(result, Ok(()));
/// ```
pub fn eval_exec_if(
    condition: &Condition,
    block: &Vec<ASTNode>,
    turtle: &mut Turtle,
    vars: &mut HashMap<String, Expression>,
) -> Result<(), ExecutionError> {
    let exec = should_execute(condition, turtle, vars)?;

    if exec {
        execute(block, turtle, vars)?;
    }

    Ok(())
}

/// Evaluates the condition and executes a `WHILE` block if the condition is true.
///
/// # Examples
///
/// ```rust
/// use std::collections::HashMap;
/// use turtle::Turtle;
/// use parser::ast::{ASTNode, Condition, Expression};
/// use interpreter::errors::ExecutionError;
///
/// let mut vars: HashMap<String, Expression> = HashMap::new();
/// let mut turtle = Turtle::new();
/// let condition = Condition::LessThan(
///    Box::new(Expression::Float(8.0)),
///    Box::new(Expression::Float(10.0)),
/// );
///
/// let block = vec![ASTNode::Command(Command::Forward(Expression::Float(100.0)))];
/// let res = eval_exec_while(&condition, &block, &mut turtle, &mut vars);
/// assert_eq!(res, Ok(()));
/// ```
pub fn eval_exec_while(
    condition: &Condition,
    block: &Vec<ASTNode>,
    turtle: &mut Turtle,
    vars: &mut HashMap<String, Expression>,
) -> Result<(), ExecutionError> {
    let mut exec = should_execute(condition, turtle, vars)?;

    while exec {
        execute(block, turtle, vars)?;

        exec = should_execute(condition, turtle, vars)?;
    }

    Ok(())
}

/// Determines if the condition is true or not.
///
/// # Examples
///
/// ```rust
/// use std::collections::HashMap;
/// use turtle::Turtle;
/// use parser::ast::{Condition, Expression};
/// use interpreter::control_flows::should_execute;
/// use interpreter::errors::ExecutionError;
///
/// let mut vars: HashMap<String, Expression> = HashMap::new();
/// let turtle = Turtle::new();
/// let condition = Condition::LessThan(
///   Box::new(Expression::Float(8.0)),
///   Box::new(Expression::Float(10.0)),
/// );
///
/// let res = should_execute(&condition, &turtle, &vars);
/// assert_eq!(res, Ok(true));
/// ```
fn should_execute(
    condition: &Condition,
    turtle: &Turtle,
    vars: &HashMap<String, Expression>,
) -> Result<bool, ExecutionError> {
    match condition {
        Condition::Equals(lhs, rhs) => comparator(lhs, rhs, |a, b| a == b, turtle, vars),
        Condition::LessThan(lhs, rhs) => comparator(lhs, rhs, |a, b| a < b, turtle, vars),
        Condition::GreaterThan(lhs, rhs) => comparator(lhs, rhs, |a, b| a > b, turtle, vars),
        Condition::And(lhs, rhs) => comparator(lhs, rhs, |a, b| a != 0.0 && b != 0.0, turtle, vars),
        Condition::Or(lhs, rhs) => comparator(lhs, rhs, |a, b| a != 0.0 || b != 0.0, turtle, vars),
    }
}
