//! Handles the execution of the parsed AST and draws the image using the
//! turtle.

use std::collections::HashMap;

use crate::parser::ast::{ASTNode, Command, ControlFlow, Expression, Query};

use super::{
    control_flows::{eval_exec_if, eval_exec_while},
    errors::{ExecutionError, ExecutionErrorKind},
    matches::match_expressions,
    turtle::Turtle,
};

/// Executes the parsed AST and draws on the image using the turtle.
pub fn execute(
    ast: &Vec<ASTNode>,
    turtle: &mut Turtle,
    vars: &mut HashMap<String, Expression>,
) -> Result<(), ExecutionError> {
    for node in ast {
        match node {
            ASTNode::Command(command) => match command {
                Command::PenDown => turtle.pen_down(),
                Command::PenUp => turtle.pen_up(),
                Command::Forward(expr) => {
                    let dist = match_expressions(expr, vars, turtle)?;
                    turtle.forward(dist);
                }
                Command::Back(expr) => {
                    let dist = match_expressions(expr, vars, turtle)?;
                    turtle.back(dist);
                }
                Command::Left(expr) => {
                    let dist = match_expressions(expr, vars, turtle)?;
                    turtle.left(dist);
                }
                Command::Right(expr) => {
                    let dist = match_expressions(expr, vars, turtle)?;
                    turtle.right(dist);
                }
                Command::SetPenColor(expr) => {
                    let color = match_expressions(expr, vars, turtle)?;
                    turtle.set_pen_color(color as usize)
                }
                Command::Turn(expr) => {
                    let degs = match_expressions(expr, vars, turtle)?;
                    turtle.turn(degs as i32);
                }
                Command::SetHeading(expr) => {
                    let degs = match_expressions(expr, vars, turtle)?;
                    turtle.set_heading(degs as i32);
                }
                Command::SetX(expr) => {
                    let x = match_expressions(expr, vars, turtle)?;
                    turtle.set_x(x);
                }
                Command::SetY(expr) => {
                    let y = match_expressions(expr, vars, turtle)?;
                    turtle.set_y(y);
                }
                Command::Make(var, expr) => {
                    // TODO: I hate this, need to refactor.
                    let var = var.to_string();
                    if let Expression::Query(query) = expr {
                        match query {
                            Query::XCor => {
                                vars.insert(var, Expression::Float(turtle.x));
                            }
                            Query::YCor => {
                                vars.insert(var, Expression::Float(turtle.y));
                            }
                            Query::Heading => {
                                vars.insert(var, Expression::Number(turtle.heading));
                            }
                            Query::Color => {
                                vars.insert(var, Expression::Usize(turtle.pen_color));
                            }
                        }
                    } else if let Expression::Float(_) = expr {
                        vars.insert(var.clone(), expr.clone());
                    } else if let Expression::Number(_) = expr {
                        vars.insert(var.clone(), expr.clone());
                    } else if let Expression::Usize(_) = expr {
                        vars.insert(var.clone(), expr.clone());
                    } else if let Expression::Math(_) = expr {
                        let val = match_expressions(expr, vars, turtle)?;
                        vars.insert(var.clone(), Expression::Float(val));
                    } else {
                        return Err(ExecutionError {
                            kind: ExecutionErrorKind::TypeError {
                                expected: "float, number, usize, query, or mathematical expression"
                                    .to_string(),
                            },
                        });
                    }
                }
                Command::AddAssign(var, expr) => {
                    let val = match_expressions(expr, vars, turtle)?;

                    if let Some(Expression::Float(curr_val)) = vars.get(var) {
                        vars.insert(var.to_string(), Expression::Float(curr_val + val));
                    } else {
                        return Err(ExecutionError {
                            kind: ExecutionErrorKind::VariableNotFound {
                                var: var.to_string(),
                            },
                        });
                    }
                }
            },
            ASTNode::ControlFlow(control_flow) => match control_flow {
                ControlFlow::If { condition, block } => {
                    eval_exec_if(condition, block, turtle, vars)?;
                }
                ControlFlow::While { condition, block } => {
                    eval_exec_while(condition, block, turtle, vars)?;
                }
            },
        }
    }

    Ok(())
}
