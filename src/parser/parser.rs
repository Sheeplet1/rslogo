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
                let expr = match_parse(&tokens, &mut (*curr_pos + 1), variables)?;
                ast.push(ASTNode::Command(Command::Forward(expr)));
            }
            "BACK" => {
                let expr = match_parse(&tokens, &mut (*curr_pos + 1), variables)?;
                ast.push(ASTNode::Command(Command::Back(expr)));
            }
            "LEFT" => {
                let expr = match_parse(&tokens, &mut (*curr_pos + 1), variables)?;
                ast.push(ASTNode::Command(Command::Left(expr)));
            }
            "RIGHT" => {
                let expr = match_parse(&tokens, &mut (*curr_pos + 1), variables)?;
                ast.push(ASTNode::Command(Command::Right(expr)));
            }
            "SETHEADING" => {
                let expr = match_parse(&tokens, &mut (*curr_pos + 1), variables)?;
                ast.push(ASTNode::Command(Command::SetHeading(expr)));
            }
            "SETX" => {
                let expr = match_parse(&tokens, &mut (*curr_pos + 1), variables)?;
                ast.push(ASTNode::Command(Command::SetX(expr)));
            }
            "SETY" => {
                let expr = match_parse(&tokens, &mut (*curr_pos + 1), variables)?;
                ast.push(ASTNode::Command(Command::SetY(expr)));
            }
            "SETPENCOLOR" => {
                let expr = match_parse(&tokens, &mut (*curr_pos + 1), variables)?;
                ast.push(ASTNode::Command(Command::SetPenColor(expr)));
            }
            "TURN" => {
                let expr = match_parse(&tokens, &mut (*curr_pos + 1), variables)?;
                ast.push(ASTNode::Command(Command::Turn(expr)));
            }
            "MAKE" => {
                *curr_pos += 1;
                let var_name = tokens[*curr_pos].trim_start_matches('"');

                let expr: Result<Expression, ParseError> =
                    match_parse(&tokens, &mut (*curr_pos + 1), variables);

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

                let expr = match_parse(&tokens, &mut (*curr_pos + 1), variables)?;

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
                let condition = parse_conditions(&tokens, &mut (*curr_pos + 1), variables)?;
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
            "TO" => {
                // Start of a procedure definition
                todo!()
            }
            "END" => {
                // End of a procedure definition
                todo!()
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
