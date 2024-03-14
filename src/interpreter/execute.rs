//! Handles the execution of the parsed AST and draws the image using the
//! turtle.

use std::collections::HashMap;

use crate::{
    errors::ExecutionError,
    parser::ast::{ASTNode, Command, ControlFlow, Expression, Query},
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
                Command::Forward(dist) => {
                    if let Expression::Float(dist) = dist {
                        turtle.forward(dist);
                    } else if let Expression::Query(query) = dist {
                        turtle.forward(match_queries(query, turtle));
                    } else if let Expression::Variable(var) = dist {
                        turtle.forward(extract_variable_value(&var, variables)?);
                    } else {
                        return Err(ExecutionError {
                            msg: format!("Forward distance must be a float. {:?}", dist),
                        });
                    }
                }
                Command::Back(dist) => {
                    if let Expression::Float(dist) = dist {
                        turtle.back(dist);
                    } else if let Expression::Query(query) = dist {
                        turtle.back(match_queries(query, turtle));
                    } else if let Expression::Variable(var) = dist {
                        turtle.back(extract_variable_value(&var, variables)?);
                    } else {
                        return Err(ExecutionError {
                            msg: format!("Back distance must be a float. {:?}", dist),
                        });
                    }
                }
                Command::Left(dist) => {
                    if let Expression::Float(dist) = dist {
                        turtle.left(dist);
                    } else if let Expression::Query(query) = dist {
                        turtle.left(match_queries(query, turtle));
                    } else if let Expression::Variable(var) = dist {
                        turtle.left(extract_variable_value(&var, variables)?);
                    } else {
                        return Err(ExecutionError {
                            msg: format!("Left distance must be a float. {:?}", dist),
                        });
                    }
                }
                Command::Right(expr) => {
                    if let Expression::Float(dist) = expr {
                        turtle.right(dist);
                    } else if let Expression::Query(query) = expr {
                        turtle.right(match_queries(query, turtle));
                    } else if let Expression::Variable(var) = expr {
                        turtle.right(extract_variable_value(&var, variables)?);
                    } else {
                        return Err(ExecutionError {
                            msg: format!("Right distance must be a float. {:?}", expr),
                        });
                    }
                }
                Command::SetPenColor(expr) => {
                    if let Expression::Usize(color) = expr {
                        turtle.set_pen_color(color).map_err(|e| ExecutionError {
                            msg: format!("Set pen color is invalid. {}", e),
                        })?;
                    } else if let Expression::Variable(var) = expr {
                        turtle
                            .set_pen_color(extract_variable_value(&var, variables)? as usize)
                            .map_err(|e| ExecutionError {
                                msg: format!("Set pen color is invalid. {}", e),
                            })?;
                    } else {
                        return Err(ExecutionError {
                            msg: format!("Set pen color must be a usize. {:?}", expr),
                        });
                    }
                }
                Command::Turn(expr) => {
                    if let Expression::Number(degrees) = expr {
                        turtle.turn(degrees);
                    } else if let Expression::Query(query) = expr {
                        turtle.turn(match_queries(query, turtle) as i32);
                    } else if let Expression::Variable(var) = expr {
                        turtle.turn(extract_variable_value(&var, variables)? as i32);
                    } else {
                        return Err(ExecutionError {
                            msg: format!("Turn must be of type i32. {:?}", expr),
                        });
                    }
                }
                Command::SetHeading(expr) => {
                    if let Expression::Number(degrees) = expr {
                        turtle.set_heading(degrees);
                    } else if let Expression::Query(query) = expr {
                        turtle.set_heading(match_queries(query, turtle) as i32);
                    } else if let Expression::Variable(var) = expr {
                        turtle.set_heading(extract_variable_value(&var, variables)? as i32);
                    } else {
                        return Err(ExecutionError {
                            msg: format!("Set heading must be of type i32. {:?}", expr),
                        });
                    }
                }
                Command::SetX(expr) => {
                    if let Expression::Float(x) = expr {
                        turtle.set_x(x);
                    } else if let Expression::Query(query) = expr {
                        turtle.set_x(match_queries(query, turtle));
                    } else if let Expression::Variable(var) = expr {
                        turtle.set_x(extract_variable_value(&var, variables)?);
                    } else {
                        return Err(ExecutionError {
                            msg: format!("Set x must be a float. {:?}", expr),
                        });
                    }
                }
                Command::SetY(expr) => {
                    if let Expression::Float(y) = expr {
                        turtle.set_y(y);
                    } else if let Expression::Query(query) = expr {
                        turtle.set_y(match_queries(query, turtle));
                    } else if let Expression::Variable(var) = expr {
                        turtle.set_y(extract_variable_value(&var, variables)?);
                    } else {
                        return Err(ExecutionError {
                            msg: format!("Set y must be a float. {:?}", expr),
                        });
                    }
                }
                Command::Make(var, expr) => {
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
                    } else {
                        return Err(ExecutionError {
                            msg: format!("Make expression must be a float or a query. {:?}", expr),
                        });
                    }
                }
                Command::AddAssign(var, expr) => {
                    let val = match expr {
                        Expression::Float(val) => val,
                        Expression::Number(val) => val as f32,
                        Expression::Usize(val) => val as f32,
                        Expression::Variable(var) => extract_variable_value(&var, variables)?,
                        Expression::Query(query) => match_queries(query, turtle),
                    };

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
                    todo!()
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
