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
    ast: Vec<ASTNode>,
    turtle: &mut Turtle,
    variables: &mut HashMap<String, Expression>,
) -> Result<(), ExecutionError> {
    println!("ast: {:#?}", ast);
    for node in ast {
        match node {
            ASTNode::Command(command) => match command {
                Command::PenDown => turtle.pen_down(),
                Command::PenUp => turtle.pen_up(),
                Command::Forward(expr) => {
                    let dist = match_expressions(expr.clone(), variables, turtle);
                    if dist.is_nan() {
                        return Err(ExecutionError {
                            msg: format!("Forward distance must be a float. {:?}", expr),
                        });
                    }
                    turtle.forward(dist);
                }
                Command::Back(expr) => {
                    let dist = match_expressions(expr.clone(), variables, turtle);
                    if dist.is_nan() {
                        return Err(ExecutionError {
                            msg: format!("Back distance must be a float. {:?}", expr),
                        });
                    }
                    turtle.back(dist);
                }
                Command::Left(expr) => {
                    let dist = match_expressions(expr.clone(), variables, turtle);
                    if dist.is_nan() {
                        return Err(ExecutionError {
                            msg: format!("Left distance must be a float. {:?}", expr),
                        });
                    }
                    turtle.left(dist);
                }
                Command::Right(expr) => {
                    let dist = match_expressions(expr.clone(), variables, turtle);
                    if dist.is_nan() {
                        return Err(ExecutionError {
                            msg: format!("Right distance must be a float. {:?}", expr),
                        });
                    }
                    turtle.right(dist);
                }
                Command::SetPenColor(expr) => {
                    let color = match_expressions(expr.clone(), variables, turtle);
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
                    let degrees = match_expressions(expr.clone(), variables, turtle);
                    if degrees.is_nan() {
                        return Err(ExecutionError {
                            msg: format!("Turn degrees must be an i32. {:?}", expr),
                        });
                    }
                    turtle.turn(degrees as i32);
                }
                Command::SetHeading(expr) => {
                    let degrees = match_expressions(expr.clone(), variables, turtle);
                    if degrees.is_nan() {
                        return Err(ExecutionError {
                            msg: format!("Set heading degrees must be an i32. {:?}", expr),
                        });
                    }
                    turtle.set_heading(degrees as i32);
                }
                Command::SetX(expr) => {
                    let x = match_expressions(expr.clone(), variables, turtle);
                    if x.is_nan() {
                        return Err(ExecutionError {
                            msg: format!("Set x must be a float. {:?}", expr),
                        });
                    }
                    turtle.set_x(x);
                }
                Command::SetY(expr) => {
                    let y = match_expressions(expr.clone(), variables, turtle);
                    if y.is_nan() {
                        return Err(ExecutionError {
                            msg: format!("Set y must be a float. {:?}", expr),
                        });
                    }
                    turtle.set_y(y);
                }
                Command::Make(var, expr) => {
                    // TODO: Refactor this
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
                        variables.insert(var, expr);
                    } else if let Expression::Number(_) = expr {
                        variables.insert(var, expr);
                    } else if let Expression::Usize(_) = expr {
                        variables.insert(var, expr);
                    } else {
                        return Err(ExecutionError {
                            msg: format!("Make expression must be a float or a query. {:?}", expr),
                        });
                    }
                }
                Command::AddAssign(var, expr) => {
                    let val = match_expressions(expr.clone(), variables, turtle);

                    if let Some(Expression::Float(curr_val)) = variables.get(&var) {
                        variables.insert(var, Expression::Float(curr_val + val));
                    } else {
                        return Err(ExecutionError {
                            msg: format!("Variable {} does not exist. Consider constructing the variable with MAKE first.", var),
                        });
                    }
                }
            },
            ASTNode::ControlFlow(control_flow) => match control_flow {
                ControlFlow::If { condition, block } => {
                    // condition: Expression: Query | Float | Number | Usize | Variable
                    // block: Vec<ASTNode>
                    // helper function to evaluate conditions and execute the block
                    eval_and_exec_block(condition, block, turtle, variables)?;
                }
                ControlFlow::While { condition, block } => {
                    todo!()
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
fn match_queries(query: Query, turtle: &Turtle) -> f32 {
    match query {
        Query::XCor => turtle.x,
        Query::YCor => turtle.y,
        Query::Heading => turtle.heading as f32,
        Query::Color => turtle.pen_color as f32,
    }
}

/// Helper function to match expressions to their values.
fn match_expressions(
    expr: Expression,
    variables: &HashMap<String, Expression>,
    turtle: &Turtle,
) -> f32 {
    match expr {
        Expression::Float(val) => val,
        Expression::Number(val) => val as f32,
        Expression::Usize(val) => val as f32,
        Expression::Query(query) => match_queries(query, turtle),
        Expression::Variable(var) => extract_variable_value(&var, variables).unwrap(),
    }
}

/// Helper function to extract the value of a variable from the `variables` hashmap.
///
/// Primarily used in the `execute` function to reduce duplicated code.
fn extract_variable_value(
    var: &str,
    variables: &HashMap<String, Expression>,
) -> Result<f32, ExecutionError> {
    if let Some(Expression::Float(val)) = variables.get(var) {
        Ok(*val)
    } else {
        Err(ExecutionError {
            msg: format!(
                "Variable {} does not exist. Consider constructing the variable with MAKE first.",
                var
            ),
        })
    }
}

/// Helper function to evaluate conditions and execute the block.
fn eval_and_exec_block(
    condition: Condition,
    block: Vec<ASTNode>,
    turtle: &mut Turtle,
    variables: &mut HashMap<String, Expression>,
) -> Result<(), ExecutionError> {
    let (val_1, val_2) = match condition {
        Condition::Equals(lhs, rhs) => {
            let lhs = match_expressions(lhs, variables, turtle);
            let rhs = match_expressions(rhs, variables, turtle);

            (lhs, rhs)
        }
        Condition::LessThan(lhs, rhs) => {
            let lhs = match_expressions(lhs, variables, turtle);
            let rhs = match_expressions(rhs, variables, turtle);

            (lhs, rhs)
        }
        Condition::GreaterThan(lhs, rhs) => {
            let lhs = match_expressions(lhs, variables, turtle);
            let rhs = match_expressions(rhs, variables, turtle);

            (lhs, rhs)
        }
    };

    println!("val_1: {}, val_2: {}", val_1, val_2);
    if val_1 == val_2 {
        println!("Condition is true. Executing block.");
        execute(block, turtle, variables)?;
    }

    Ok(())
}
