//! Handles parsing the Logo script into an Abstract Syntax Tree (AST).
//!
//! The Logo script is tokenised into a vector of tokens, which are then parsed
//! into ASTNode and Expression types. The ASTNode type is used to represent the
//! Abstract Syntax Tree (AST) of the Logo script, and the Expression type is
//! used to represent the different types of expressions that can be parsed from
//! the Logo script, such as floats, numbers, queries, and variables.

use std::collections::HashMap;

use crate::{errors::ParseError, parser::ast::ControlFlow};

use super::{
    ast::ASTNode,
    ast::Command,
    ast::Expression,
    helpers::{match_parse, parse_conditional_blocks, parse_conditions},
};

/// Parse tokens into an Abstract Syntax Tree (AST).
///
/// # Examples
///
/// ```rust
/// use std::collections::HashMap;
///
/// // Tokens is generated from the tokenize_script function.
/// tokens = vec!["PENDOWN", "FORWARD", "\"100"]
///
/// let mut variables: HashMap<String, Expression> = HashMap::new();
/// let ast = parse_tokens(tokens, &mut variables)?;
///
/// assert_eq!(ast, vec![ASTNode::Command(Command::PenDown),
///         ASTNode::Command(Command::Forward(Expression::Float(100.0)))]);
/// ```
pub fn parse_tokens(
    tokens: Vec<&str>,
    curr_pos: &mut usize,
    variables: &mut HashMap<String, Expression>,
) -> Result<Vec<ASTNode>, ParseError> {
    let mut ast = Vec::new();

    while *curr_pos < tokens.len() {
        match tokens[*curr_pos] {
            "PENUP" => {
                ast.push(ASTNode::Command(Command::PenUp));
            }
            "PENDOWN" => {
                ast.push(ASTNode::Command(Command::PenDown));
            }
            "FORWARD" => {
                *curr_pos += 1;
                let expr = match_parse(&tokens, curr_pos, variables)?;
                ast.push(ASTNode::Command(Command::Forward(expr)));
            }
            "BACK" => {
                *curr_pos += 1;
                let expr = match_parse(&tokens, curr_pos, variables)?;
                ast.push(ASTNode::Command(Command::Back(expr)));
            }
            "LEFT" => {
                *curr_pos += 1;
                let expr = match_parse(&tokens, curr_pos, variables)?;
                ast.push(ASTNode::Command(Command::Left(expr)));
            }
            "RIGHT" => {
                *curr_pos += 1;
                let expr = match_parse(&tokens, curr_pos, variables)?;
                ast.push(ASTNode::Command(Command::Right(expr)));
            }
            "SETHEADING" => {
                *curr_pos += 1;
                let expr = match_parse(&tokens, curr_pos, variables)?;
                ast.push(ASTNode::Command(Command::SetHeading(expr)));
            }
            "SETX" => {
                *curr_pos += 1;
                let expr = match_parse(&tokens, curr_pos, variables)?;
                ast.push(ASTNode::Command(Command::SetX(expr)));
            }
            "SETY" => {
                *curr_pos += 1;
                let expr = match_parse(&tokens, curr_pos, variables)?;
                ast.push(ASTNode::Command(Command::SetY(expr)));
            }
            "SETPENCOLOR" => {
                *curr_pos += 1;
                let expr = match_parse(&tokens, curr_pos, variables)?;
                ast.push(ASTNode::Command(Command::SetPenColor(expr)));
            }
            "TURN" => {
                *curr_pos += 1;
                let expr = match_parse(&tokens, curr_pos, variables)?;
                ast.push(ASTNode::Command(Command::Turn(expr)));
            }
            "MAKE" => {
                *curr_pos += 1;
                let var_name = tokens[*curr_pos].trim_start_matches('"');

                *curr_pos += 1;
                let expr: Result<Expression, ParseError> =
                    match_parse(&tokens, curr_pos, variables);

                // Now that expr is of type `Expression`, we can insert it into the
                // variables HashMap, making it easier on the execution phase.
                match expr {
                    Ok(expr) => {
                        variables.insert(var_name.to_string(), expr.clone());
                        ast.push(ASTNode::Command(Command::Make(var_name.to_string(), expr)));
                    }
                    Err(e) => return Err(e),
                };
            }
            "ADDASSIGN" => {
                // ADDASSIGN can only work on variables
                *curr_pos += 1;
                if !tokens[*curr_pos].starts_with('"') {
                    return Err(ParseError {
                        msg: format!("Invalid expression for ADDASSIGN: {:?}\nExpressions for ADDASSIGN should start with \"", tokens[*curr_pos]),
                    });
                }

                let var_name = tokens[*curr_pos].trim_start_matches('"');
                if !variables.contains_key(var_name) {
                    return Err(ParseError {
                        msg: format!("Variable not found for ADDASSIGN: {:?}", var_name),
                    });
                }

                *curr_pos += 1;
                let expr = match_parse(&tokens, curr_pos, variables)?;

                ast.push(ASTNode::Command(Command::AddAssign(
                    var_name.to_string(),
                    expr,
                )));
            }
            "IF" => {
                *curr_pos += 1; // Skip the IF token
                let condition = parse_conditions(&tokens, &mut *curr_pos, variables)?;
                let block = parse_conditional_blocks(&tokens, &mut *curr_pos, variables)?;
                ast.push(ASTNode::ControlFlow(ControlFlow::If { condition, block }));
            }
            "WHILE" => {
                *curr_pos += 1; // Skip the WHILE token
                let condition = parse_conditions(&tokens, &mut *curr_pos, variables)?;
                let block = parse_conditional_blocks(&tokens, &mut *curr_pos, variables)?;
                ast.push(ASTNode::ControlFlow(ControlFlow::While {
                    condition,
                    block,
                }));
            }
            "]" => {
                // This is the end of a conditional block, we can skip this token
                // and return the ast directly.
                return Ok(ast);
            }
            _ => {
                return Err(ParseError {
                    msg: format!("Failed to parse expression: {:?}", tokens[*curr_pos]),
                });
            }
        }
        *curr_pos += 1
    }

    Ok(ast)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::parser::ast::{Condition, Query};

    use super::*;

    #[test]
    fn test_parse_tokens() {
        let mut variables: HashMap<String, Expression> = HashMap::new();
        let mut curr_pos = 0;

        let tokens = vec!["PENUP", "PENDOWN", "FORWARD", "\"100"];
        let ast = parse_tokens(tokens, &mut curr_pos, &mut variables).unwrap();

        assert_eq!(
            ast,
            vec![
                ASTNode::Command(Command::PenUp),
                ASTNode::Command(Command::PenDown),
                ASTNode::Command(Command::Forward(Expression::Float(100.0)))
            ]
        );
    }

    #[test]
    fn test_parse_tokens_with_make() {
        let mut variables: HashMap<String, Expression> = HashMap::new();
        let mut curr_pos = 0;

        let tokens = vec!["MAKE", "\"x", "\"100"];
        let ast = parse_tokens(tokens, &mut curr_pos, &mut variables).unwrap();

        assert_eq!(
            ast,
            vec![ASTNode::Command(Command::Make(
                "x".to_string(),
                Expression::Float(100.0)
            ))]
        );
    }

    #[test]
    fn test_parse_tokens_with_add_assign() {
        let mut variables: HashMap<String, Expression> = HashMap::new();
        let mut curr_pos = 0;

        let tokens = vec!["MAKE", "\"x", "\"100", "ADDASSIGN", "\"x", "\"100"];
        let ast = parse_tokens(tokens, &mut curr_pos, &mut variables).unwrap();

        assert_eq!(
            ast,
            vec![
                ASTNode::Command(Command::Make("x".to_string(), Expression::Float(100.0))),
                ASTNode::Command(Command::AddAssign(
                    "x".to_string(),
                    Expression::Float(100.0)
                ))
            ]
        );
    }

    #[test]
    fn test_parse_tokens_with_if() {
        let mut variables: HashMap<String, Expression> = HashMap::new();
        let mut curr_pos = 0;

        let tokens = vec!["IF", "EQ", "XCOR", "\"100", "[", "FORWARD", "\"100", "]"];
        let ast = parse_tokens(tokens, &mut curr_pos, &mut variables).unwrap();

        assert_eq!(
            ast,
            vec![ASTNode::ControlFlow(ControlFlow::If {
                condition: Condition::Equals(
                    Expression::Query(Query::XCor),
                    Expression::Float(100.0)
                ),
                block: vec![ASTNode::Command(Command::Forward(Expression::Float(100.0)))]
            })]
        );
    }

    #[test]
    fn test_parse_tokens_with_while() {
        let mut variables: HashMap<String, Expression> = HashMap::new();
        let mut curr_pos = 0;

        let tokens = vec!["WHILE", "EQ", "XCOR", "\"100", "[", "FORWARD", "\"100", "]"];
        let ast = parse_tokens(tokens, &mut curr_pos, &mut variables).unwrap();

        assert_eq!(
            ast,
            vec![ASTNode::ControlFlow(ControlFlow::While {
                condition: Condition::Equals(
                    Expression::Query(Query::XCor),
                    Expression::Float(100.0)
                ),
                block: vec![ASTNode::Command(Command::Forward(Expression::Float(100.0)))]
            })]
        );
    }

    #[test]
    fn test_parse_tokens_with_invalid_expression() {
        let mut variables: HashMap<String, Expression> = HashMap::new();
        let mut curr_pos = 0;

        let tokens = vec!["FORWARD", "\"10x"];
        let ast = parse_tokens(tokens, &mut curr_pos, &mut variables).unwrap_err();

        assert_eq!(
            ast,
            ParseError {
                msg: "Failed to parse expression: \"\\\"10x\"".to_string()
            }
        );
    }
}
