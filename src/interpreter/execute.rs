//! Handles the execution of the parsed AST and draws the image using the
//! turtle.

use std::collections::HashMap;

use crate::{
    errors::ExecutionError,
    parser::ast::{ASTNode, Command, ControlFlow, Expression, Query},
};

use super::{
    control_flows::{eval_exec_if, eval_exec_while},
    matches::match_expressions,
    turtle::Turtle,
};

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
                    } else {
                        variables.insert(var, expr.clone());
                        println!("{:?}", variables);
                    }

                    // else if let Expression::Float(_) = expr {
                    //     variables.insert(var.clone(), expr.clone());
                    // } else if let Expression::Number(_) = expr {
                    //     variables.insert(var.clone(), expr.clone());
                    // } else if let Expression::Usize(_) = expr {
                    //     variables.insert(var.clone(), expr.clone());
                    // } else {
                    //     return Err(ExecutionError {
                    //         msg: format!("Make expression must be a float or a query. {:?}", expr),
                    //     });
                    // }
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
