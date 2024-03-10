use std::collections::HashMap;

use crate::{errors::ParseError, parser::ast::Query};

use super::{ast::ASTNode, ast::Command, ast::Expression};

/// Tokenise the lg script into a vector of tokens. Each token is an instruction
/// or value.
pub fn tokenize_script(contents: &str) -> Vec<&str> {
    let tokens: Vec<&str> = contents
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .filter(|line| !line.starts_with("//"))
        .collect();

    tokens
        .iter()
        .flat_map(|line| line.split_whitespace())
        .collect()
}

/// Parse tokens into an Abstract Syntax Tree (AST).
pub fn parse_tokens(
    tokens: Vec<&str>,
    variables: &mut HashMap<String, Expression>,
) -> Result<Vec<ASTNode>, ParseError> {
    let mut ast = Vec::new();
    let mut curr_pos = 0;

    while curr_pos < tokens.len() {
        match tokens[curr_pos] {
            "PENUP" => {
                ast.push(ASTNode::Command(Command::PenUp));
            }
            "PENDOWN" => {
                ast.push(ASTNode::Command(Command::PenDown));
            }
            "FORWARD" => {
                curr_pos += 1;
                if tokens[curr_pos].starts_with('"') {
                    let expr = parse_expression(&tokens, curr_pos)?;
                    ast.push(ASTNode::Command(Command::Forward(Expression::Float(expr))));
                } else if tokens[curr_pos].starts_with(':') {
                    let expr = parse_variable(&tokens, curr_pos, variables)?;
                    ast.push(ASTNode::Command(Command::Forward(expr)));
                } else {
                    let expr = parse_query(&tokens, curr_pos)?;
                    ast.push(ASTNode::Command(Command::Forward(Expression::Query(expr))));
                }
            }
            "BACK" => {
                curr_pos += 1;
                if tokens[curr_pos].starts_with('"') {
                    let expr = parse_expression(&tokens, curr_pos)?;
                    ast.push(ASTNode::Command(Command::Back(Expression::Float(expr))));
                } else if tokens[curr_pos].starts_with(':') {
                    let expr = parse_variable(&tokens, curr_pos, variables)?;
                    ast.push(ASTNode::Command(Command::Back(expr)));
                } else {
                    let expr = parse_query(&tokens, curr_pos)?;
                    ast.push(ASTNode::Command(Command::Back(Expression::Query(expr))));
                }
            }
            "LEFT" => {
                curr_pos += 1;
                if tokens[curr_pos].starts_with('"') {
                    let expr = parse_expression(&tokens, curr_pos)?;
                    ast.push(ASTNode::Command(Command::Left(Expression::Float(expr))));
                } else if tokens[curr_pos].starts_with(':') {
                    let expr = parse_variable(&tokens, curr_pos, variables)?;
                    ast.push(ASTNode::Command(Command::Left(expr)));
                } else {
                    let expr = parse_query(&tokens, curr_pos)?;
                    ast.push(ASTNode::Command(Command::Left(Expression::Query(expr))));
                }
            }
            "RIGHT" => {
                curr_pos += 1;
                if tokens[curr_pos].starts_with('"') {
                    let expr = parse_expression(&tokens, curr_pos)?;
                    ast.push(ASTNode::Command(Command::Right(Expression::Float(expr))));
                } else if tokens[curr_pos].starts_with(':') {
                    let expr = parse_variable(&tokens, curr_pos, variables)?;
                    ast.push(ASTNode::Command(Command::Right(expr)));
                } else {
                    let expr = parse_query(&tokens, curr_pos)?;
                    ast.push(ASTNode::Command(Command::Right(Expression::Query(expr))));
                }
            }
            "SETHEADING" => {
                curr_pos += 1;
                if tokens[curr_pos].starts_with('"') {
                    let expr = parse_expression(&tokens, curr_pos)? as i32;
                    ast.push(ASTNode::Command(Command::SetHeading(Expression::Number(
                        expr,
                    ))));
                } else if tokens[curr_pos].starts_with(':') {
                    let expr = parse_variable(&tokens, curr_pos, variables)?;
                    ast.push(ASTNode::Command(Command::SetHeading(expr)));
                } else {
                    let expr = parse_query(&tokens, curr_pos)?;
                    ast.push(ASTNode::Command(Command::SetHeading(Expression::Query(
                        expr,
                    ))));
                }
            }
            "SETX" => {
                curr_pos += 1;
                if tokens[curr_pos].starts_with('"') {
                    let expr = parse_expression(&tokens, curr_pos)?;
                    ast.push(ASTNode::Command(Command::SetX(Expression::Float(expr))));
                } else if tokens[curr_pos].starts_with(':') {
                    let expr = parse_variable(&tokens, curr_pos, variables)?;
                    ast.push(ASTNode::Command(Command::SetX(expr)));
                } else {
                    let expr = parse_query(&tokens, curr_pos)?;
                    ast.push(ASTNode::Command(Command::SetX(Expression::Query(expr))));
                }
            }
            "SETY" => {
                curr_pos += 1;
                if tokens[curr_pos].starts_with('"') {
                    let expr = parse_expression(&tokens, curr_pos)?;
                    ast.push(ASTNode::Command(Command::SetY(Expression::Float(expr))));
                } else if tokens[curr_pos].starts_with(':') {
                    let expr = parse_variable(&tokens, curr_pos, variables)?;
                    ast.push(ASTNode::Command(Command::SetY(expr)));
                } else {
                    let expr = parse_query(&tokens, curr_pos)?;
                    ast.push(ASTNode::Command(Command::SetY(Expression::Query(expr))));
                }
            }
            "SETPENCOLOR" => {
                curr_pos += 1;
                if tokens[curr_pos].starts_with('"') {
                    let expr = parse_expression(&tokens, curr_pos)? as usize;
                    ast.push(ASTNode::Command(Command::SetPenColor(Expression::Usize(
                        expr,
                    ))));
                } else if tokens[curr_pos].starts_with(':') {
                    let expr = parse_variable(&tokens, curr_pos, variables)?;
                    match expr {
                        Expression::Float(val) => {
                            ast.push(ASTNode::Command(Command::SetPenColor(Expression::Usize(
                                val as usize,
                            ))));
                        }
                        _ => {
                            return Err(ParseError {
                                msg: format!(
                                    "Parsing error for SETPENCOLOR: {:?}",
                                    tokens[curr_pos]
                                ),
                            });
                        }
                    }
                } else {
                    let expr = parse_query(&tokens, curr_pos)?;
                    ast.push(ASTNode::Command(Command::SetPenColor(Expression::Query(
                        expr,
                    ))));
                }
            }
            "TURN" => {
                curr_pos += 1;
                if tokens[curr_pos].starts_with('"') {
                    let expr = parse_expression(&tokens, curr_pos)? as i32;
                    ast.push(ASTNode::Command(Command::Turn(Expression::Number(expr))));
                } else if tokens[curr_pos].starts_with(':') {
                    let expr = parse_variable(&tokens, curr_pos, variables)?;
                    ast.push(ASTNode::Command(Command::Turn(expr)));
                } else {
                    let expr = parse_query(&tokens, curr_pos)?;
                    ast.push(ASTNode::Command(Command::Turn(Expression::Query(expr))));
                }
            }
            "MAKE" => {
                curr_pos += 1;
                let var_name = tokens[curr_pos].trim_start_matches('"');

                curr_pos += 1;
                let token = tokens[curr_pos];

                // Expression may either be a float or a query.
                let expr: Result<Expression, ParseError> = if token.starts_with('"') {
                    parse_expression(&tokens, curr_pos).map(Expression::Float)
                } else {
                    let query = match token {
                        "XCOR" => Query::XCor,
                        "YCOR" => Query::YCor,
                        "HEADING" => Query::Heading,
                        "COLOR" => Query::Color,
                        _ => {
                            return Err(ParseError {
                                msg: format!("Invalid query expression: {:?}", token),
                            });
                        }
                    };
                    Ok(Expression::Query(query))
                };

                match expr {
                    Ok(expr) => variables.insert(var_name.to_string(), expr),
                    Err(e) => return Err(e),
                };
            }
            _ => {
                return Err(ParseError {
                    msg: format!("Parsing error for token: {:?}", tokens[curr_pos]),
                });
            }
        }
        curr_pos += 1
    }

    Ok(ast)
}

////////////////////////////////////////////////////////////////////////////////
////////////////////////////// PARSING FUNCTIONS ///////////////////////////////
////////////////////////////////////////////////////////////////////////////////

/// Parse an expression from a token. An expression may be a variable, a query
/// or a value.
///
/// If it is a variable, then it will be looked up in the variables HashMap.
///
/// If it is a value, then it will be parsed into a f32.
///
/// If it is a query, then it will be parsed into a f32.

/// Parse an expression from a token. This expression will always result in
/// a f32 value.
fn parse_expression(tokens: &[&str], pos: usize) -> Result<f32, ParseError> {
    if tokens[pos].starts_with('"') {
        tokens[pos]
            .trim_start_matches('"')
            .parse::<f32>()
            .map_err(|_| ParseError {
                msg: format!("Failed to parse expression: {:?}", tokens[pos]),
            })
    } else {
        Err(ParseError {
            msg: format!("Cannot parse invalid expression: {:?}", tokens[pos]),
        })
    }
}

/// Parse a query from a token. A query is a special type of expression that
/// returns a value from the turtle state.
fn parse_query(tokens: &[&str], pos: usize) -> Result<Query, ParseError> {
    let query = match tokens[pos] {
        "XCOR" => Query::XCor,
        "YCOR" => Query::YCor,
        "HEADING" => Query::Heading,
        "COLOR" => Query::Color,
        _ => {
            return Err(ParseError {
                msg: format!("Invalid query expression: {:?}", tokens[pos]),
            });
        }
    };
    Ok(query)
}

/// Parses a stored variable from a token to its corresponding expression.
fn parse_variable(
    tokens: &[&str],
    pos: usize,
    variables: &HashMap<String, Expression>,
) -> Result<Expression, ParseError> {
    let var_name = tokens[pos].trim_start_matches(':');
    // variables { x: Query(Xcor), y: Query(Ycor), distance: Expression::Float(50),  }
    match variables.get(var_name) {
        Some(expr) => Ok(expr.clone()),
        None => Err(ParseError {
            msg: format!("Variable not found: {:?}", var_name),
        }),
    }
}