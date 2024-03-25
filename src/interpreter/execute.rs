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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use unsvg::Image;

    use crate::parser::ast::{Command, Condition, Expression, Math, Query};

    use super::*;

    #[test]
    fn test_execute_pen_down() {
        let mut image = Image::new(100, 100);
        let mut turtle = Turtle::new(&mut image);
        let mut vars = HashMap::new();

        let ast = vec![ASTNode::Command(Command::PenDown)];

        execute(&ast, &mut turtle, &mut vars).unwrap();

        assert!(turtle.pen_down);
    }

    #[test]
    fn test_execute_pen_up() {
        let mut image = Image::new(100, 100);
        let mut turtle = Turtle::new(&mut image);
        let mut vars = HashMap::new();

        let ast = vec![
            ASTNode::Command(Command::PenDown),
            ASTNode::Command(Command::PenUp),
        ];

        execute(&ast, &mut turtle, &mut vars).unwrap();

        assert!(!turtle.pen_down);
    }

    #[test]
    fn test_execute_forward() {
        let mut image = Image::new(100, 100);
        let mut turtle = Turtle::new(&mut image);
        let mut vars = HashMap::new();

        let ast = vec![ASTNode::Command(Command::Forward(Expression::Float(30.0)))];

        execute(&ast, &mut turtle, &mut vars).unwrap();

        assert_eq!(turtle.y, 20.0);
    }

    #[test]
    fn test_execute_back() {
        let mut image = Image::new(100, 100);
        let mut turtle = Turtle::new(&mut image);
        let mut vars = HashMap::new();

        let ast = vec![ASTNode::Command(Command::Back(Expression::Float(30.0)))];

        execute(&ast, &mut turtle, &mut vars).unwrap();

        assert_eq!(turtle.y, 80.0);
    }

    #[test]
    fn test_execute_left() {
        let mut image = Image::new(100, 100);
        let mut turtle = Turtle::new(&mut image);
        let mut vars = HashMap::new();

        let ast = vec![ASTNode::Command(Command::Left(Expression::Float(30.0)))];

        execute(&ast, &mut turtle, &mut vars).unwrap();

        assert_eq!(turtle.x, 20.0);
    }

    #[test]
    fn test_execute_right() {
        let mut image = Image::new(100, 100);
        let mut turtle = Turtle::new(&mut image);
        let mut vars = HashMap::new();

        let ast = vec![ASTNode::Command(Command::Right(Expression::Float(30.0)))];

        execute(&ast, &mut turtle, &mut vars).unwrap();

        assert_eq!(turtle.x, 80.0);
    }

    #[test]
    fn test_execute_set_pen_color() {
        let mut image = Image::new(100, 100);
        let mut turtle = Turtle::new(&mut image);
        let mut vars = HashMap::new();

        let ast = vec![ASTNode::Command(Command::SetPenColor(Expression::Usize(1)))];

        execute(&ast, &mut turtle, &mut vars).unwrap();

        assert_eq!(turtle.pen_color, 1);
    }

    #[test]
    fn test_execute_turn() {
        let mut image = Image::new(100, 100);
        let mut turtle = Turtle::new(&mut image);
        let mut vars = HashMap::new();

        let ast = vec![ASTNode::Command(Command::Turn(Expression::Number(30)))];

        execute(&ast, &mut turtle, &mut vars).unwrap();

        assert_eq!(turtle.heading, 30);
    }

    #[test]
    fn test_execute_set_heading() {
        let mut image = Image::new(100, 100);
        let mut turtle = Turtle::new(&mut image);
        let mut vars = HashMap::new();

        let ast = vec![ASTNode::Command(Command::SetHeading(Expression::Number(
            30,
        )))];

        execute(&ast, &mut turtle, &mut vars).unwrap();

        assert_eq!(turtle.heading, 30);
    }

    #[test]
    fn test_execute_set_x() {
        let mut image = Image::new(100, 100);
        let mut turtle = Turtle::new(&mut image);
        let mut vars = HashMap::new();

        turtle.set_y(50.0);

        let ast = vec![ASTNode::Command(Command::SetX(Expression::Float(30.0)))];

        execute(&ast, &mut turtle, &mut vars).unwrap();

        assert_eq!(turtle.x, 30.0);
    }

    #[test]
    fn test_execute_set_y() {
        let mut image = Image::new(100, 100);
        let mut turtle = Turtle::new(&mut image);
        let mut vars = HashMap::new();

        turtle.set_x(50.0);

        let ast = vec![ASTNode::Command(Command::SetY(Expression::Float(30.0)))];

        execute(&ast, &mut turtle, &mut vars).unwrap();

        assert_eq!(turtle.y, 30.0);
    }

    #[test]
    fn test_execute_make_queries() {
        let mut image = Image::new(100, 100);
        let mut turtle = Turtle::new(&mut image);
        let mut vars = HashMap::new();

        let ast = vec![
            ASTNode::Command(Command::Make(
                "x".to_string(),
                Expression::Query(Query::XCor),
            )),
            ASTNode::Command(Command::Make(
                "y".to_string(),
                Expression::Query(Query::YCor),
            )),
            ASTNode::Command(Command::Make(
                "heading".to_string(),
                Expression::Query(Query::Heading),
            )),
            ASTNode::Command(Command::Make(
                "color".to_string(),
                Expression::Query(Query::Color),
            )),
        ];

        execute(&ast, &mut turtle, &mut vars).unwrap();

        assert_eq!(vars.get("x").unwrap(), &Expression::Float(50.0));
        assert_eq!(vars.get("y").unwrap(), &Expression::Float(50.0));
        assert_eq!(vars.get("heading").unwrap(), &Expression::Number(0));
        assert_eq!(vars.get("color").unwrap(), &Expression::Usize(7));
    }

    #[test]
    fn test_execute_make_other() {
        let mut image = Image::new(100, 100);
        let mut turtle = Turtle::new(&mut image);
        let mut vars = HashMap::new();

        let ast = vec![
            ASTNode::Command(Command::Make("float".to_string(), Expression::Float(30.0))),
            ASTNode::Command(Command::Make("number".to_string(), Expression::Number(30))),
            ASTNode::Command(Command::Make("usize".to_string(), Expression::Usize(1))),
            ASTNode::Command(Command::Make(
                "math".to_string(),
                Expression::Math(Box::new(Math::Add(
                    Expression::Float(10.0),
                    Expression::Float(10.0),
                ))),
            )),
        ];

        execute(&ast, &mut turtle, &mut vars).unwrap();

        assert_eq!(vars.get("float").unwrap(), &Expression::Float(30.0));
        assert_eq!(vars.get("number").unwrap(), &Expression::Number(30));
        assert_eq!(vars.get("usize").unwrap(), &Expression::Usize(1));
        assert_eq!(vars.get("math").unwrap(), &Expression::Float(20.0));
    }

    #[test]
    fn test_execute_make_err() {
        // Only one case where there will be an error is when the expression is
        // a variable.
        let mut image = Image::new(100, 100);
        let mut turtle = Turtle::new(&mut image);
        let mut vars = HashMap::new();

        let ast = vec![ASTNode::Command(Command::Make(
            "x".to_string(),
            Expression::Variable("y".to_string()),
        ))];

        let result = execute(&ast, &mut turtle, &mut vars);

        assert!(result.is_err());
    }

    #[test]
    fn test_execute_add_assign() {
        let mut image = Image::new(100, 100);
        let mut turtle = Turtle::new(&mut image);
        let mut vars = HashMap::new();
        vars.insert("x".to_string(), Expression::Float(10.0));

        let ast = vec![ASTNode::Command(Command::AddAssign(
            "x".to_string(),
            Expression::Float(10.0),
        ))];

        execute(&ast, &mut turtle, &mut vars).unwrap();

        assert_eq!(vars.get("x").unwrap(), &Expression::Float(20.0));
    }

    #[test]
    fn test_execute_add_assign_err() {
        let mut image = Image::new(100, 100);
        let mut turtle = Turtle::new(&mut image);
        let mut vars = HashMap::new();

        let ast = vec![ASTNode::Command(Command::AddAssign(
            "x".to_string(),
            Expression::Float(10.0),
        ))];

        let result = execute(&ast, &mut turtle, &mut vars);

        assert!(result.is_err());
    }

    #[test]
    fn test_execute_if() {
        let mut image = Image::new(100, 100);
        let mut turtle = Turtle::new(&mut image);
        let mut vars = HashMap::new();
        vars.insert("x".to_string(), Expression::Float(10.0));

        let ast = vec![ASTNode::ControlFlow(ControlFlow::If {
            condition: Condition::Equals(
                Expression::Float(10.0),
                Expression::Math(Box::new(Math::Add(
                    Expression::Float(5.0),
                    Expression::Float(5.0),
                ))),
            ),
            block: vec![ASTNode::Command(Command::AddAssign(
                "x".to_string(),
                Expression::Float(10.0),
            ))],
        })];

        execute(&ast, &mut turtle, &mut vars).unwrap();

        assert_eq!(vars.get("x").unwrap(), &Expression::Float(20.0));
    }

    #[test]
    fn test_execute_while() {
        let mut image = Image::new(100, 100);
        let mut turtle = Turtle::new(&mut image);
        let mut vars = HashMap::new();
        vars.insert("x".to_string(), Expression::Float(10.0));

        let ast = vec![ASTNode::ControlFlow(ControlFlow::While {
            condition: Condition::LessThan(
                Expression::Variable("x".to_string()),
                Expression::Float(20.0),
            ),
            block: vec![ASTNode::Command(Command::AddAssign(
                "x".to_string(),
                Expression::Float(1.0),
            ))],
        })];

        execute(&ast, &mut turtle, &mut vars).unwrap();

        assert_eq!(vars.get("x").unwrap(), &Expression::Float(20.0));
    }
}
