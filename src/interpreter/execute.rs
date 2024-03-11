use std::collections::HashMap;

use crate::{
    errors::ExecutionError,
    parser::ast::{ASTNode, Command, Expression, Query},
};

use super::turtle::Turtle;

/// Execute instructions in the AST on the turtle to draw an image.
pub fn execute(
    ast: Vec<ASTNode>,
    turtle: &mut Turtle,
    variables: &mut HashMap<String, Expression>,
) -> Result<(), ExecutionError> {
    // println!("ast: {:#?}", ast);
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
                    } else {
                        return Err(ExecutionError {
                            msg: "Forward distance must be a float.".to_string(),
                        });
                    }
                }
                Command::Back(dist) => {
                    if let Expression::Float(dist) = dist {
                        turtle.back(dist);
                    } else if let Expression::Query(query) = dist {
                        turtle.back(match_queries(query, turtle));
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
                    } else {
                        return Err(ExecutionError {
                            msg: "Right distance must be a float.".to_string(),
                        });
                    }
                }
                Command::SetPenColor(expr) => {
                    if let Expression::Usize(color) = expr {
                        turtle.set_pen_color(color).map_err(|e| ExecutionError {
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
                    } else {
                        return Err(ExecutionError {
                            msg: "Turn degrees must be of type i32.".to_string(),
                        });
                    }
                }
                Command::SetHeading(expr) => {
                    if let Expression::Number(degrees) = expr {
                        turtle.set_heading(degrees);
                    } else if let Expression::Query(query) = expr {
                        turtle.set_heading(match_queries(query, turtle) as i32);
                    } else {
                        return Err(ExecutionError {
                            msg: "Set heading degrees must of type i32.".to_string(),
                        });
                    }
                }
                Command::SetX(expr) => {
                    if let Expression::Float(x) = expr {
                        turtle.set_x(x);
                    } else {
                        return Err(ExecutionError {
                            msg: "Set x must be a float.".to_string(),
                        });
                    }
                }
                Command::SetY(expr) => {
                    if let Expression::Float(y) = expr {
                        turtle.set_y(y);
                    } else {
                        return Err(ExecutionError {
                            msg: "Set y must be a float.".to_string(),
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
                            msg: "Invalid expression for MAKE command.".to_string(),
                        });
                    }
                }
            },
        }
    }

    Ok(())
}

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
