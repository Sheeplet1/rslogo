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
                        match query {
                            Query::XCor => {
                                turtle.forward(turtle.x);
                            }
                            Query::YCor => {
                                turtle.forward(turtle.y);
                            }
                            Query::Heading => {
                                turtle.forward(turtle.heading as f32);
                            }
                            Query::Color => {
                                turtle.forward(turtle.pen_color as f32);
                            }
                        }
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
                        match query {
                            Query::XCor => {
                                turtle.back(turtle.x);
                            }
                            Query::YCor => {
                                turtle.back(turtle.y);
                            }
                            Query::Heading => {
                                turtle.back(turtle.heading as f32);
                            }
                            Query::Color => {
                                turtle.back(turtle.pen_color as f32);
                            }
                        }
                    } else {
                        return Err(ExecutionError {
                            // msg: "Back distance must be a float".to_string(),
                            msg: format!("Back distance must be a float. {:?}", dist),
                        });
                    }
                }
                Command::Left(dist) => {
                    if let Expression::Float(dist) = dist {
                        turtle.left(dist);
                    } else if let Expression::Query(query) = dist {
                        match query {
                            Query::XCor => {
                                turtle.left(turtle.x);
                            }
                            Query::YCor => {
                                turtle.left(turtle.y);
                            }
                            Query::Heading => {
                                turtle.left(turtle.heading as f32);
                            }
                            Query::Color => {
                                turtle.left(turtle.pen_color as f32);
                            }
                        }
                    } else {
                        return Err(ExecutionError {
                            msg: format!("Left distance must be a float. {:?}", dist),
                        });
                    }
                }
                Command::Right(expr) => {
                    if let Expression::Float(dist) = expr {
                        turtle.right(dist);
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
                        match query {
                            Query::Heading => {
                                turtle.turn(turtle.heading);
                            }
                            _ => {
                                return Err(ExecutionError {
                                    msg: "Invalid query for TURN command.".to_string(),
                                });
                            }
                        }
                    } else {
                        return Err(ExecutionError {
                            msg: "Turn degrees must be of type i32.".to_string(),
                        });
                    }
                }
                Command::SetHeading(expr) => {
                    if let Expression::Number(degrees) = expr {
                        turtle.set_heading(degrees);
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
                    // if let Expression::Float(val) = expr {
                    //     variables.insert(var, expr);
                    // } else if Expression::Query(query) = expr {
                    //     match query {
                    //         Query::XCor => {
                    //             let x = turtle.x;
                    //             variables.insert(var, Expression::Float(x));
                    //         }
                    //         Query::YCor => {
                    //             let y = turtle.y;
                    //             variables.insert(var, Expression::Float(y));
                    //         }
                    //         Query::Heading => {
                    //             let heading = turtle.heading;
                    //             variables.insert(var, Expression::Number(heading));
                    //         }
                    //         Query::Color => {
                    //             let color = turtle.pen_color;
                    //             variables.insert(var, Expression::Float(color as f32));
                    //         }
                    //     }
                    // } else {
                    //     return Err(ExecutionError {
                    //         msg: "Invalid expression for MAKE command.".to_string(),
                    //     });
                    // }
                }
            },
        }
    }

    Ok(())
}
