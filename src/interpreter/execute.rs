//! Handles the execution of the parsed AST and draws the image using the
//! turtle.

use std::collections::HashMap;

use crate::{
    errors::ExecutionError,
    parser::ast::{ASTNode, Command, Condition, ControlFlow, Expression, Query},
};

use super::turtle::Turtle;

/// Executes the parsed AST and draws on the image using the turtle.
pub fn execute(
    ast: &Vec<ASTNode>,
    turtle: &mut Turtle,
    variables: &mut HashMap<String, Expression>,
) -> Result<(), ExecutionError> {
    for node in ast {
        match node {
            ASTNode::Command(command) => match command {
                Command::PenDown => turtle.pen_down(),
                Command::PenUp => turtle.pen_up(),
                Command::Forward(expr) => {
                    let dist = match_expressions(expr, variables, turtle)?;
                    if dist.is_nan() {
                        return Err(ExecutionError {
                            msg: format!("Forward distance must be a float. {:?}", expr),
                        });
                    }
                    turtle.forward(dist);
                }
                Command::Back(expr) => {
                    let dist = match_expressions(expr, variables, turtle)?;
                    if dist.is_nan() {
                        return Err(ExecutionError {
                            msg: format!("Back distance must be a float. {:?}", expr),
                        });
                    }
                    turtle.back(dist);
                }
                Command::Left(expr) => {
                    let dist = match_expressions(expr, variables, turtle)?;
                    if dist.is_nan() {
                        return Err(ExecutionError {
                            msg: format!("Left distance must be a float. {:?}", expr),
                        });
                    }
                    turtle.left(dist);
                }
                Command::Right(expr) => {
                    let dist = match_expressions(expr, variables, turtle)?;
                    if dist.is_nan() {
                        return Err(ExecutionError {
                            msg: format!("Right distance must be a float. {:?}", expr),
                        });
                    }
                    turtle.right(dist);
                }
                Command::SetPenColor(expr) => {
                    let color = match_expressions(expr, variables, turtle)?;
                    if color.is_nan() {
                        return Err(ExecutionError {
                            msg: format!("Set pen color must be an usize. {:?}", expr),
                        });
                    }
                    turtle
                        .set_pen_color(color as usize)
                        .map_err(|e| ExecutionError { msg: e.to_string() })?;
                }
                Command::Turn(expr) => {
                    let degrees = match_expressions(expr, variables, turtle)?;
                    if degrees.is_nan() {
                        return Err(ExecutionError {
                            msg: format!("Turn degrees must be an i32. {:?}", expr),
                        });
                    }
                    turtle.turn(degrees as i32);
                }
                Command::SetHeading(expr) => {
                    let degrees = match_expressions(expr, variables, turtle)?;
                    if degrees.is_nan() {
                        return Err(ExecutionError {
                            msg: format!("Set heading degrees must be an i32. {:?}", expr),
                        });
                    }
                    turtle.set_heading(degrees as i32);
                }
                Command::SetX(expr) => {
                    let x = match_expressions(expr, variables, turtle)?;
                    if x.is_nan() {
                        return Err(ExecutionError {
                            msg: format!("Set x must be a float. {:?}", expr),
                        });
                    }
                    turtle.set_x(x);
                }
                Command::SetY(expr) => {
                    let y = match_expressions(expr, variables, turtle)?;
                    if y.is_nan() {
                        return Err(ExecutionError {
                            msg: format!("Set y must be a float. {:?}", expr),
                        });
                    }
                    turtle.set_y(y);
                }
                Command::Make(var, expr) => {
                    // TODO: Refactor this
                    let var = var.to_string();
                    if let Expression::Query(query) = expr {
                        match query {
                            Query::XCor => {
                                variables.insert(var, Expression::Float(turtle.x));
                            }
                            Query::YCor => {
                                variables.insert(var, Expression::Float(turtle.y));
                            }
                            Query::Heading => {
                                variables.insert(var, Expression::Number(turtle.heading));
                            }
                            Query::Color => {
                                variables.insert(var, Expression::Usize(turtle.pen_color));
                            }
                        }
                    } else if let Expression::Float(_) = expr {
                        variables.insert(var.clone(), expr.clone());
                    } else if let Expression::Number(_) = expr {
                        variables.insert(var.clone(), expr.clone());
                    } else if let Expression::Usize(_) = expr {
                        variables.insert(var.clone(), expr.clone());
                    } else {
                        return Err(ExecutionError {
                            msg: format!("Make expression must be a float or a query. {:?}", expr),
                        });
                    }
                }
                Command::AddAssign(var, expr) => {
                    let val = match_expressions(expr, variables, turtle)?;

                    if let Some(Expression::Float(curr_val)) = variables.get(var) {
                        variables.insert(var.to_string(), Expression::Float(curr_val + val));
                    } else {
                        return Err(ExecutionError {
                            msg: format!("Variable {} does not exist. Consider constructing the variable with MAKE first.", var),
                        });
                    }
                }
            },
            ASTNode::ControlFlow(control_flow) => match control_flow {
                ControlFlow::If { condition, block } => {
                    eval_exec_if(condition, block, turtle, variables)?;
                }
                ControlFlow::While { condition, block } => {
                    eval_exec_while(condition, block, turtle, variables)?;
                }
            },
        }
    }

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////
/////////////////////////////// HELPER FUNCTIONS ///////////////////////////////
////////////////////////////////////////////////////////////////////////////////

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
fn match_expressions(
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
fn eval_exec_if(
    condition: &Condition,
    block: &Vec<ASTNode>,
    turtle: &mut Turtle,
    variables: &mut HashMap<String, Expression>,
) -> Result<(), ExecutionError> {
    let should_execute = match condition {
        Condition::Equals(lhs, rhs) => comparator(lhs, rhs, |a, b| a == b, turtle, variables)?,
        Condition::LessThan(lhs, rhs) => comparator(lhs, rhs, |a, b| a < b, turtle, variables)?,
        Condition::GreaterThan(lhs, rhs) => comparator(lhs, rhs, |a, b| a > b, turtle, variables)?,
    };

    if should_execute {
        execute(block, turtle, variables)?;
    }

    Ok(())
}

fn eval_exec_while(
    condition: &Condition,
    block: &Vec<ASTNode>,
    turtle: &mut Turtle,
    variables: &mut HashMap<String, Expression>,
) -> Result<(), ExecutionError> {
    let mut should_execute = match condition {
        Condition::Equals(ref lhs, ref rhs) => {
            comparator(lhs, rhs, |lhs, rhs| lhs == rhs, turtle, variables)
        }
        Condition::LessThan(ref lhs, ref rhs) => {
            comparator(lhs, rhs, |lhs, rhs| lhs < rhs, turtle, variables)
        }
        Condition::GreaterThan(ref lhs, ref rhs) => {
            comparator(lhs, rhs, |lhs, rhs| lhs > rhs, turtle, variables)
        }
    }?;

    while should_execute {
        execute(block, turtle, variables)?;

        should_execute = match condition {
            Condition::Equals(ref lhs, ref rhs) => {
                comparator(lhs, rhs, |lhs, rhs| lhs == rhs, turtle, variables)?
            }
            Condition::LessThan(ref lhs, ref rhs) => {
                comparator(lhs, rhs, |lhs, rhs| lhs < rhs, turtle, variables)?
            }
            Condition::GreaterThan(ref lhs, ref rhs) => {
                comparator(lhs, rhs, |lhs, rhs| lhs > rhs, turtle, variables)?
            }
        };
    }

    Ok(())
}
