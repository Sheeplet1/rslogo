use crate::errors::ParseError;

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
pub fn parse_tokens(tokens: Vec<&str>) -> Result<Vec<ASTNode>, ParseError> {
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
                let expr = parse_expression(&tokens, curr_pos)?;
                ast.push(ASTNode::Command(Command::Forward(Expression::Float(expr))));
            }
            "BACK" => {
                curr_pos += 1;
                let expr = parse_expression(&tokens, curr_pos)?;
                ast.push(ASTNode::Command(Command::Back(Expression::Float(expr))));
            }
            "LEFT" => {
                curr_pos += 1;
                let expr = parse_expression(&tokens, curr_pos)?;
                ast.push(ASTNode::Command(Command::Left(Expression::Float(expr))));
            }
            "RIGHT" => {
                curr_pos += 1;
                let expr = parse_expression(&tokens, curr_pos)?;
                ast.push(ASTNode::Command(Command::Right(Expression::Float(expr))));
            }
            "SETHEADING" => {
                curr_pos += 1;
                let expr: i32 = parse_expression(&tokens, curr_pos)? as i32;
                ast.push(ASTNode::Command(Command::SetHeading(Expression::Number(
                    expr,
                ))));
            }
            "SETX" => {
                curr_pos += 1;
                let expr = parse_expression(&tokens, curr_pos)?;
                ast.push(ASTNode::Command(Command::SetX(Expression::Float(expr))));
            }
            "SETY" => {
                curr_pos += 1;
                let expr = parse_expression(&tokens, curr_pos)?;
                ast.push(ASTNode::Command(Command::SetY(Expression::Float(expr))));
            }
            "SETPENCOLOR" => {
                curr_pos += 1;
                let expr = parse_expression(&tokens, curr_pos)? as usize;
                ast.push(ASTNode::Command(Command::SetPenColor(Expression::Usize(
                    expr,
                ))));
            }
            "TURN" => {
                curr_pos += 1;
                let expr: i32 = parse_expression(&tokens, curr_pos)? as i32;
                ast.push(ASTNode::Command(Command::Turn(Expression::Number(expr))));
            }
            _ => {
                return Err(ParseError {
                    msg: format!("Parsing error for token: {:?}", tokens[curr_pos]),
                });
            }
        }
        curr_pos += 1
    }

    println!("ast: {:#?}", ast);
    Ok(ast)
}

/// Parse an expression from a token. An expression may be a variable or
/// a value, but the resulting value will always be a float.
fn parse_expression(tokens: &[&str], pos: usize) -> Result<f32, ParseError> {
    // TODO: If tokens[pos] starts with ':', then it is variable.
    // If tokens[pos] starts with '"', then it is a expression.
    if tokens[pos].starts_with(':') {
        todo!()
    } else if tokens[pos].starts_with('"') {
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
