//! Control flow execution functions.
//!
//! Responsible for evaluating conditions and executing the block if the
//! condition is true.

use std::collections::HashMap;

use crate::parser::ast::{ASTNode, Condition, Expression};

use super::{errors::ExecutionError, execute::execute, matches::match_expressions, turtle::Turtle};

/// Compares two expressions using a given comparator.
///
/// # Examples
///
/// ```rust
/// use std::collections::HashMap;
/// use turtle::Turtle;
/// use parser::ast::{Condition, Expression};
/// use interpreter::errors::ExecutionError;
/// use unsvg::Image;
///
///
/// let mut vars: HashMap<String, Expression> = HashMap::new();
/// let mut image = Image::new(100, 100);
/// let turtle = Turtle::new(&mut image);
///
/// let lhs = Expression::Float(8.0);
/// let rhs = Expression::Float(10.0);
///
/// let res = comparator(&lhs, &rhs, |a, b| a < b, &turtle, &vars).unwrap();
/// assert!(res);
/// ```
fn comparator(
    lhs: &Expression,
    rhs: &Expression,
    comparator: fn(f32, f32) -> bool,
    turtle: &Turtle,
    vars: &HashMap<String, Expression>,
) -> Result<bool, ExecutionError> {
    let lhs_val = match_expressions(lhs, vars, turtle)?;
    let rhs_val = match_expressions(rhs, vars, turtle)?;
    Ok(comparator(lhs_val, rhs_val))
}

/// Evaluates the condition and executes an `IF` block if the condition is true.
///
/// # Examples
/// ```rust
/// use std::collections::HashMap;
/// use turtle::Turtle;
/// use parser::ast::{ASTNode, Condition, Expression};
/// use interpreter::control_flows::eval_exec_if;
/// use interpreter::errors::ExecutionError;
/// use unsvg::Image;
///
/// let mut vars: HashMap<String, Expression> = HashMap::new();
/// let mut image = Image::new(100, 100);
/// let mut turtle = Turtle::new(&mut image);
///
/// let condition = Condition::LessThan(
///   Expression::Float(8.0),
///   Expression::Float(10.0),
/// );
///
/// let block = vec![ASTNode::Command(Command::Forward(Expression::Float(100.0)))];
/// let res = eval_exec_if(&condition, &block, &mut turtle, &mut vars).unwrap();
/// assert!(res.is_ok());
/// ```
pub fn eval_exec_if(
    condition: &Condition,
    block: &Vec<ASTNode>,
    turtle: &mut Turtle,
    vars: &mut HashMap<String, Expression>,
) -> Result<(), ExecutionError> {
    let exec = should_execute(condition, turtle, vars)?;

    if exec {
        execute(block, turtle, vars)?;
    }

    Ok(())
}

/// Evaluates the condition and executes a `WHILE` block if the condition is true.
///
/// # Examples
///
/// ```rust
/// use std::collections::HashMap;
/// use turtle::Turtle;
/// use parser::ast::{ASTNode, Condition, Expression};
/// use interpreter::errors::ExecutionError;
///
/// let mut vars: HashMap<String, Expression> = HashMap::new();
/// let mut image = Image::new(100, 100);
/// let mut turtle = Turtle::new(&mut image);
/// let condition = Condition::LessThan(
///     Expression::Float(8.0),
///     Expression::Float(10.0),
/// );
///
/// let block = vec![ASTNode::Command(Command::Forward(Expression::Float(100.0)))];
/// let res = eval_exec_while(&condition, &block, &mut turtle, &mut vars).unwrap();
/// assert!(res.is_ok());
/// ```
pub fn eval_exec_while(
    condition: &Condition,
    block: &Vec<ASTNode>,
    turtle: &mut Turtle,
    vars: &mut HashMap<String, Expression>,
) -> Result<(), ExecutionError> {
    let mut exec = should_execute(condition, turtle, vars)?;

    while exec {
        execute(block, turtle, vars)?;

        exec = should_execute(condition, turtle, vars)?;
    }

    Ok(())
}

/// Determines if the condition is true or not.
///
/// # Examples
///
/// ```rust
/// use std::collections::HashMap;
/// use turtle::Turtle;
/// use parser::ast::{Condition, Expression};
/// use interpreter::control_flows::should_execute;
/// use interpreter::errors::ExecutionError;
///
/// let mut vars: HashMap<String, Expression> = HashMap::new();
/// let mut image = Image::new(100, 100);
/// let mut turtle = Turtle::new(&mut image);
/// let condition = Condition::LessThan(
///     Expression::Float(8.0),
///     Expression::Float(10.0),
/// );
///
/// let res = should_execute(&condition, &turtle, &vars).unwrap();
/// assert!(res);
/// ```
fn should_execute(
    condition: &Condition,
    turtle: &Turtle,
    vars: &HashMap<String, Expression>,
) -> Result<bool, ExecutionError> {
    match condition {
        Condition::Equals(lhs, rhs) => comparator(lhs, rhs, |a, b| a == b, turtle, vars),
        Condition::LessThan(lhs, rhs) => comparator(lhs, rhs, |a, b| a < b, turtle, vars),
        Condition::GreaterThan(lhs, rhs) => comparator(lhs, rhs, |a, b| a > b, turtle, vars),
        Condition::And(lhs, rhs) => comparator(lhs, rhs, |a, b| a != 0.0 && b != 0.0, turtle, vars),
        Condition::Or(lhs, rhs) => comparator(lhs, rhs, |a, b| a != 0.0 || b != 0.0, turtle, vars),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use unsvg::Image;

    use crate::parser::ast::{ASTNode, Command, Condition, Expression};

    use super::*;

    #[test]
    fn test_comparator() {
        let vars: HashMap<String, Expression> = HashMap::new();
        let mut image = Image::new(100, 100);
        let turtle = Turtle::new(&mut image);

        let lhs = Expression::Float(8.0);
        let rhs = Expression::Float(10.0);

        let res = comparator(&lhs, &rhs, |a, b| a < b, &turtle, &vars).unwrap();
        assert!(res);
    }

    #[test]
    fn test_if_true() {
        let mut vars: HashMap<String, Expression> = HashMap::new();
        let mut image = Image::new(100, 100);
        let mut turtle = Turtle::new(&mut image);

        let condition = Condition::Equals(Expression::Float(1.0), Expression::Float(1.0));
        let block = vec![ASTNode::Command(Command::PenDown)];

        let res = eval_exec_if(&condition, &block, &mut turtle, &mut vars);
        assert!(res.is_ok());
        assert!(turtle.pen_down);
    }

    #[test]
    fn test_if_false() {
        let mut vars: HashMap<String, Expression> = HashMap::new();
        let mut image = Image::new(100, 100);
        let mut turtle = Turtle::new(&mut image);

        let condition = Condition::Equals(Expression::Float(1.0), Expression::Float(2.0));
        let block = vec![ASTNode::Command(Command::PenDown)];

        let res = eval_exec_if(&condition, &block, &mut turtle, &mut vars);
        assert!(res.is_ok());
        assert!(!turtle.pen_down);
    }

    #[test]
    fn test_while_executes_correctly() {
        let mut vars = HashMap::new();
        vars.insert("counter".to_string(), Expression::Float(0.0));

        let condition = Condition::LessThan(
            Expression::Variable("counter".to_string()),
            Expression::Float(3.0),
        );

        let block = vec![
            ASTNode::Command(Command::Forward(Expression::Float(10.0))),
            ASTNode::Command(Command::Right(Expression::Float(10.0))),
            ASTNode::Command(Command::AddAssign(
                "counter".to_string(),
                Expression::Float(1.0),
            )),
        ];

        let mut image = Image::new(100, 100);
        let mut turtle = Turtle::new(&mut image);
        turtle.pen_down = true;

        let result = eval_exec_while(&condition, &block, &mut turtle, &mut vars);
        assert!(result.is_ok());

        // Check if turtle has moved correctly and counter variable has increased
        assert_eq!(turtle.y, 20.0);
        assert_eq!(turtle.x, 80.0);

        match vars.get("counter") {
            Some(Expression::Float(val)) => assert_eq!(*val, 3.0),
            _ => panic!("Counter variable was not incremented correctly"),
        }
    }

    #[test]
    fn test_while_does_not_execute() {
        let mut vars = HashMap::new();
        vars.insert("counter".to_string(), Expression::Float(3.0));

        let condition = Condition::LessThan(
            Expression::Variable("counter".to_string()),
            Expression::Float(3.0),
        );

        let block = vec![
            ASTNode::Command(Command::Forward(Expression::Float(10.0))),
            ASTNode::Command(Command::Right(Expression::Float(10.0))),
            ASTNode::Command(Command::AddAssign(
                "counter".to_string(),
                Expression::Float(1.0),
            )),
        ];

        let mut image = Image::new(100, 100);
        let mut turtle = Turtle::new(&mut image);
        turtle.pen_down = true;

        let result = eval_exec_while(&condition, &block, &mut turtle, &mut vars);
        assert!(result.is_ok());

        // Check if turtle has moved correctly and counter variable has increased
        assert_eq!(turtle.y, 50.0);
        assert_eq!(turtle.x, 50.0);

        match vars.get("counter") {
            Some(Expression::Float(val)) => assert_eq!(*val, 3.0),
            _ => panic!("Counter variable was not incremented correctly"),
        }
    }

    #[test]
    fn test_should_execute_gt() {
        let vars: HashMap<String, Expression> = HashMap::new();
        let mut image = Image::new(100, 100);
        let turtle = Turtle::new(&mut image);

        let condition = Condition::GreaterThan(Expression::Float(8.0), Expression::Float(10.0));
        let res = should_execute(&condition, &turtle, &vars).unwrap();
        assert!(!res);
    }

    #[test]
    fn test_should_execute_and() {
        let vars: HashMap<String, Expression> = HashMap::new();
        let mut image = Image::new(100, 100);
        let turtle = Turtle::new(&mut image);

        let condition = Condition::And(Expression::Float(1.0), Expression::Float(0.0));

        let res = should_execute(&condition, &turtle, &vars).unwrap();
        assert!(!res);
    }

    #[test]
    fn test_should_execute_or() {
        let vars: HashMap<String, Expression> = HashMap::new();
        let mut image = Image::new(100, 100);
        let turtle = Turtle::new(&mut image);

        let condition = Condition::Or(Expression::Float(1.0), Expression::Float(0.0));

        let res = should_execute(&condition, &turtle, &vars).unwrap();
        assert!(res);
    }
}
