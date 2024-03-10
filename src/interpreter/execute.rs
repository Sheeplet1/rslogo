use crate::{
    errors::ExecutionError,
    parser::ast::{ASTNode, Command, Expression},
};

use super::turtle::Turtle;

/// Execute instructions in the AST on the turtle to draw an image.
pub fn execute(ast: Vec<ASTNode>, turtle: &mut Turtle) -> Result<(), ExecutionError> {
    println!("execute ast: {:?}", ast);

    for node in ast {
        match node {
            ASTNode::Command(command) => match command {
                Command::PenDown => turtle.pen_down(),
                Command::PenUp => turtle.pen_up(),
                Command::Forward(dist) => {
                    if let Expression::Float(dist) = dist {
                        println!("forward we go!");
                        turtle.forward(dist);
                    } else {
                        return Err(ExecutionError {
                            msg: "Forward distance must be a float.".to_string(),
                        });
                    }
                }
                Command::Back(dist) => {
                    if let Expression::Float(dist) = dist {
                        turtle.back(dist);
                    } else {
                        return Err(ExecutionError {
                            msg: "Back distance must be a float.".to_string(),
                        });
                    }
                }
                Command::Left(dist) => {
                    if let Expression::Float(dist) = dist {
                        turtle.left(dist);
                    } else {
                        return Err(ExecutionError {
                            msg: "Left distance must be a float.".to_string(),
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
                        turtle
                            .set_pen_color(color)
                            .map_err(|e| ExecutionError { msg: e.msg })?;
                    } else {
                        return Err(ExecutionError {
                            msg: "Set pen color must be a usize.".to_string(),
                        });
                    }
                }
                Command::Turn(expr) => {
                    if let Expression::Number(degrees) = expr {
                        turtle.turn(degrees);
                    } else {
                        return Err(ExecutionError {
                            msg: "Turn degrees must be a number.".to_string(),
                        });
                    }
                }
                Command::SetHeading(expr) => {
                    if let Expression::Number(degrees) = expr {
                        turtle.set_heading(degrees);
                    } else {
                        return Err(ExecutionError {
                            msg: "Set heading degrees must be a number.".to_string(),
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
            },
        }
    }

    Ok(())
}
