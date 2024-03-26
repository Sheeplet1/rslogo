//! Handles parsing the Logo script into an Abstract Syntax Tree (AST).
//!
//! The Logo script is tokenised into a vector of tokens, which is then parsed
//! into ASTNode and Expression types. The ASTNode type is used to represent the
//! Abstract Syntax Tree (AST) of the Logo script, and the Expression type is
//! used to represent the different types of expressions that can be parsed from
//! the Logo script, such as floats, numbers, queries, and vars.

use std::collections::HashMap;

use crate::ast::{ASTNode, Command, ControlFlow, Expression};

use super::{
    errors::{ParseError, ParseErrorKind},
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
/// let mut vars: HashMap<String, Expression> = HashMap::new();
/// let ast = parse_tokens(tokens, &mut vars).unwrap();
///
/// assert_eq!(ast, vec![ASTNode::Command(Command::PenDown),
///         ASTNode::Command(Command::Forward(Expression::Float(100.0)))]);
/// ```
pub fn parse_tokens(
    tokens: Vec<&str>,
    curr_pos: &mut usize,
    vars: &mut HashMap<String, Expression>,
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
                let expr = match_parse(&tokens, curr_pos, vars)?;
                ast.push(ASTNode::Command(Command::Forward(expr)));
            }
            "BACK" => {
                *curr_pos += 1;
                let expr = match_parse(&tokens, curr_pos, vars)?;
                ast.push(ASTNode::Command(Command::Back(expr)));
            }
            "LEFT" => {
                *curr_pos += 1;
                let expr = match_parse(&tokens, curr_pos, vars)?;
                ast.push(ASTNode::Command(Command::Left(expr)));
            }
            "RIGHT" => {
                *curr_pos += 1;
                let expr = match_parse(&tokens, curr_pos, vars)?;
                ast.push(ASTNode::Command(Command::Right(expr)));
            }
            "SETHEADING" => {
                *curr_pos += 1;
                let expr = match_parse(&tokens, curr_pos, vars)?;
                ast.push(ASTNode::Command(Command::SetHeading(expr)));
            }
            "SETX" => {
                *curr_pos += 1;
                let expr = match_parse(&tokens, curr_pos, vars)?;
                ast.push(ASTNode::Command(Command::SetX(expr)));
            }
            "SETY" => {
                *curr_pos += 1;
                let expr = match_parse(&tokens, curr_pos, vars)?;
                ast.push(ASTNode::Command(Command::SetY(expr)));
            }
            "SETPENCOLOR" => {
                *curr_pos += 1;
                let expr = match_parse(&tokens, curr_pos, vars)?;

                if let Expression::Float(color) = expr {
                    if !(0..=15).contains(&(color as usize)) {
                        return Err(ParseError {
                            kind: ParseErrorKind::InvalidSyntax {
                                msg: "Colour index must be between 0 and 15 inclusive.".to_string(),
                            },
                        });
                    }
                }

                ast.push(ASTNode::Command(Command::SetPenColor(expr)));
            }
            "TURN" => {
                *curr_pos += 1;
                let expr = match_parse(&tokens, curr_pos, vars)?;
                ast.push(ASTNode::Command(Command::Turn(expr)));
            }
            "MAKE" => {
                *curr_pos += 1;
                let var_name = tokens[*curr_pos].trim_start_matches('"');

                *curr_pos += 1;
                let expr: Result<Expression, ParseError> = match_parse(&tokens, curr_pos, vars);

                match expr {
                    Ok(expr) => {
                        vars.insert(var_name.to_string(), expr.clone());
                        ast.push(ASTNode::Command(Command::Make(var_name.to_string(), expr)));
                    }
                    Err(_) => unreachable!(),
                };
            }
            "ADDASSIGN" => {
                // ADDASSIGN can only work on vars
                *curr_pos += 1;
                if !tokens[*curr_pos].starts_with('"') {
                    return Err(ParseError {
                        kind: ParseErrorKind::InvalidSyntax {
                            msg: "ADDASSIGN can only work on vars".to_string(),
                        },
                    });
                }

                let var_name = tokens[*curr_pos].trim_start_matches('"');
                if !vars.contains_key(var_name) {
                    return Err(ParseError {
                        kind: ParseErrorKind::VariableNotFound {
                            var: var_name.to_string(),
                        },
                    });
                }

                *curr_pos += 1;
                let expr = match_parse(&tokens, curr_pos, vars)?;

                ast.push(ASTNode::Command(Command::AddAssign(
                    var_name.to_string(),
                    expr,
                )));
            }
            "IF" => {
                *curr_pos += 1; // Skip the IF token
                let condition = parse_conditions(&tokens, &mut *curr_pos, vars)?;
                let block = parse_conditional_blocks(&tokens, &mut *curr_pos, vars)?;
                ast.push(ASTNode::ControlFlow(ControlFlow::If { condition, block }));
            }
            "WHILE" => {
                *curr_pos += 1; // Skip the WHILE token
                let condition = parse_conditions(&tokens, &mut *curr_pos, vars)?;
                let block = parse_conditional_blocks(&tokens, &mut *curr_pos, vars)?;
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
                unimplemented!();
            }
            "END" => {
                unimplemented!();
            }
            _ => {
                return Err(ParseError {
                    kind: ParseErrorKind::UnexpectedToken {
                        token: tokens[*curr_pos].to_string(),
                    },
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

    use crate::ast::Condition;

    use super::*;

    #[test]
    fn test_parse_basic_tokens() {
        let mut vars: HashMap<String, Expression> = HashMap::new();
        let mut curr_pos = 0;

        let tokens = vec![
            "PENUP",
            "PENDOWN",
            "FORWARD",
            "\"100",
            "BACK",
            "\"100",
            "LEFT",
            "\"100",
            "RIGHT",
            "\"100",
            "SETHEADING",
            "\"100",
            "SETX",
            "\"100",
            "SETY",
            "\"100",
            "SETPENCOLOR",
            "\"1",
            "TURN",
            "\"100",
        ];
        let ast = parse_tokens(tokens, &mut curr_pos, &mut vars).unwrap();

        assert_eq!(
            ast,
            vec![
                ASTNode::Command(Command::PenUp),
                ASTNode::Command(Command::PenDown),
                ASTNode::Command(Command::Forward(Expression::Float(100.0))),
                ASTNode::Command(Command::Back(Expression::Float(100.0))),
                ASTNode::Command(Command::Left(Expression::Float(100.0))),
                ASTNode::Command(Command::Right(Expression::Float(100.0))),
                ASTNode::Command(Command::SetHeading(Expression::Float(100.0))),
                ASTNode::Command(Command::SetX(Expression::Float(100.0))),
                ASTNode::Command(Command::SetY(Expression::Float(100.0))),
                ASTNode::Command(Command::SetPenColor(Expression::Float(1.0))),
                ASTNode::Command(Command::Turn(Expression::Float(100.0))),
            ]
        );
    }

    #[test]
    fn test_parse_pen_color_err() {
        let mut vars: HashMap<String, Expression> = HashMap::new();
        let mut curr_pos = 0;

        let tokens = vec!["SETPENCOLOR", "\"16"];
        let ast = parse_tokens(tokens, &mut curr_pos, &mut vars);

        assert_eq!(
            ast,
            Err(ParseError {
                kind: ParseErrorKind::InvalidSyntax {
                    msg: "Colour index must be between 0 and 15 inclusive.".to_string()
                }
            })
        );
    }

    #[test]
    fn test_parse_make() {
        let mut vars: HashMap<String, Expression> = HashMap::new();
        let mut curr_pos = 0;

        let tokens = vec!["MAKE", "\"x", "\"100"];
        let ast = parse_tokens(tokens, &mut curr_pos, &mut vars).unwrap();

        assert_eq!(
            ast,
            vec![ASTNode::Command(Command::Make(
                "x".to_string(),
                Expression::Float(100.0)
            ),)]
        );
    }

    #[test]
    fn test_parse_add_assign() {
        let mut vars: HashMap<String, Expression> = HashMap::new();
        vars.insert("x".to_string(), Expression::Float(100.0));
        let mut curr_pos = 0;

        let tokens = vec!["ADDASSIGN", "\"x", "\"100"];
        let ast = parse_tokens(tokens, &mut curr_pos, &mut vars).unwrap();

        assert_eq!(
            ast,
            vec![ASTNode::Command(Command::AddAssign(
                "x".to_string(),
                Expression::Float(100.0)
            ),)]
        );
    }

    #[test]
    fn test_parse_add_assign_not_var() {
        let mut vars: HashMap<String, Expression> = HashMap::new();
        let mut curr_pos = 0;

        let tokens = vec!["ADDASSIGN", "x", "\"100"];
        let ast = parse_tokens(tokens, &mut curr_pos, &mut vars);

        assert_eq!(
            ast,
            Err(ParseError {
                kind: ParseErrorKind::InvalidSyntax {
                    msg: "ADDASSIGN can only work on vars".to_string()
                }
            })
        );
    }

    #[test]
    fn test_parse_add_assign_no_var() {
        let mut vars: HashMap<String, Expression> = HashMap::new();
        let mut curr_pos = 0;

        let tokens = vec!["ADDASSIGN", "\"x", "\"100"];
        let ast = parse_tokens(tokens, &mut curr_pos, &mut vars);

        assert_eq!(
            ast,
            Err(ParseError {
                kind: ParseErrorKind::VariableNotFound {
                    var: "x".to_string()
                }
            })
        );
    }

    #[test]
    fn test_parse_if() {
        let mut vars: HashMap<String, Expression> = HashMap::new();
        let mut curr_pos = 0;

        let tokens = vec!["IF", "EQ", "\"100", "\"100", "[", "FORWARD", "\"100", "]"];
        let ast = parse_tokens(tokens, &mut curr_pos, &mut vars).unwrap();

        assert_eq!(
            ast,
            vec![ASTNode::ControlFlow(ControlFlow::If {
                condition: Condition::Equals(Expression::Float(100.0), Expression::Float(100.0)),
                block: vec![ASTNode::Command(Command::Forward(Expression::Float(100.0)))]
            })]
        );
    }

    #[test]
    fn test_parse_while() {
        let mut vars: HashMap<String, Expression> = HashMap::new();
        let mut curr_pos = 0;

        let tokens = vec![
            "WHILE", "EQ", "\"100", "\"100", "[", "FORWARD", "\"100", "]",
        ];
        let ast = parse_tokens(tokens, &mut curr_pos, &mut vars).unwrap();

        assert_eq!(
            ast,
            vec![ASTNode::ControlFlow(ControlFlow::While {
                condition: Condition::Equals(Expression::Float(100.0), Expression::Float(100.0)),
                block: vec![ASTNode::Command(Command::Forward(Expression::Float(100.0)))]
            })]
        );
    }

    #[test]
    fn test_parse_unexpected_token() {
        let mut vars: HashMap<String, Expression> = HashMap::new();
        let mut curr_pos = 0;

        let tokens = vec!["INVALID"];
        let ast = parse_tokens(tokens, &mut curr_pos, &mut vars);

        assert_eq!(
            ast,
            Err(ParseError {
                kind: ParseErrorKind::UnexpectedToken {
                    token: "INVALID".to_string()
                }
            })
        );
    }
}
