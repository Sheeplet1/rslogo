use std::collections::HashMap;

use crate::{
    errors::ExecutionError,
    parser::ast::{ASTNode, Condition, Expression},
};

use super::{execute::execute, matches::match_expressions, turtle::Turtle};

// TODO: Make this comparator generic so that it can handle both f32 and bool.
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

/// Helper function to evaluate conditions and execute the block.
pub fn eval_exec_if(
    condition: &Condition,
    block: &Vec<ASTNode>,
    turtle: &mut Turtle,
    variables: &mut HashMap<String, Expression>,
) -> Result<(), ExecutionError> {
    // TODO: Refactor should_execute
    let exec = should_execute(condition, turtle, variables)?;

    if exec {
        execute(block, turtle, variables)?;
    }

    Ok(())
}

pub fn eval_exec_while(
    condition: &Condition,
    block: &Vec<ASTNode>,
    turtle: &mut Turtle,
    variables: &mut HashMap<String, Expression>,
) -> Result<(), ExecutionError> {
    // TODO: Refactor should_execute
    let mut exec = should_execute(condition, turtle, variables)?;

    while exec {
        execute(block, turtle, variables)?;

        exec = should_execute(condition, turtle, variables)?;
    }

    Ok(())
}

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
            todo!()
        }
        Condition::Or(lhs, rhs) => {
            todo!()
        }
        Condition::NotEqual(lhs, rhs) => {
            todo!()
        }
    }
}
