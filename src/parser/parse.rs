//! Handles parsing the Logo script into an Abstract Syntax Tree (AST).
//!
//! The Logo script is tokenised into a vector of tokens, which are then parsed
//! into ASTNode and Expression types. The ASTNode type is used to represent the
//! Abstract Syntax Tree (AST) of the Logo script, and the Expression type is
//! used to represent the different types of expressions that can be parsed from
//! the Logo script, such as floats, numbers, queries, and variables.

use std::collections::HashMap;

use crate::{
    errors::ParseError,
    parser::ast::{ControlFlow, Query},
};

use super::{
    ast::ASTNode,
    ast::Expression,
    ast::{Command, Condition},
};

/// Tokenises an Logo script into a vector of tokens. Each token is an instruction
/// or value.
///
/// # Examples
///
/// Consider the Logo script:
/// ```Logo
/// PENDOWN
///
/// SETPENCOLOR "1
/// FORWARD "100
/// ```
///
/// Tokenising this script would result in the following vector:
/// ```rust
/// vec!["PENDOWN", "SETPENCOLOR" "\"1", "FORWARD" "\"100"]
/// ````
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
                let expr = match_parse(&tokens, curr_pos, variables)?;
                ast.push(ASTNode::Command(Command::Forward(expr)));
            }
            "BACK" => {
                curr_pos += 1;
                let expr = match_parse(&tokens, curr_pos, variables)?;
                ast.push(ASTNode::Command(Command::Back(expr)));
            }
            "LEFT" => {
                curr_pos += 1;
                let expr = match_parse(&tokens, curr_pos, variables)?;
                ast.push(ASTNode::Command(Command::Left(expr)));
            }
            "RIGHT" => {
                curr_pos += 1;
                let expr = match_parse(&tokens, curr_pos, variables)?;
                ast.push(ASTNode::Command(Command::Right(expr)));
            }
            "SETHEADING" => {
                curr_pos += 1;
                let expr = match_parse(&tokens, curr_pos, variables)?;

                // Because all expressions are parsed as floats, we need to convert
                // the float to an i32 for the SETHEADING command.
                match expr {
                    Expression::Float(val) => ast.push(ASTNode::Command(Command::SetHeading(
                        Expression::Number(val as i32),
                    ))),
                    Expression::Variable(var) => ast.push(ASTNode::Command(Command::SetHeading(
                        Expression::Variable(var),
                    ))),
                    Expression::Query(query) => ast.push(ASTNode::Command(Command::SetHeading(
                        Expression::Query(query),
                    ))),
                    _ => {
                        return Err(ParseError {
                            msg: format!(
                                "Failed to parse expression for SETHEADING: {:?}",
                                tokens[curr_pos]
                            ),
                        });
                    }
                }
            }
            "SETX" => {
                curr_pos += 1;
                let expr = match_parse(&tokens, curr_pos, variables)?;
                ast.push(ASTNode::Command(Command::SetX(expr)));
            }
            "SETY" => {
                curr_pos += 1;
                let expr = match_parse(&tokens, curr_pos, variables)?;
                ast.push(ASTNode::Command(Command::SetY(expr)));
            }
            "SETPENCOLOR" => {
                curr_pos += 1;
                let expr = match_parse(&tokens, curr_pos, variables)?;

                // Because all expressions are parsed as floats, we need to convert
                // the float to an usize for the SETPENCOLOR command.
                match expr {
                    Expression::Float(val) => ast.push(ASTNode::Command(Command::SetPenColor(
                        Expression::Usize(val as usize),
                    ))),
                    Expression::Variable(var) => ast.push(ASTNode::Command(Command::SetPenColor(
                        Expression::Variable(var),
                    ))),
                    Expression::Query(query) => ast.push(ASTNode::Command(Command::SetPenColor(
                        Expression::Query(query),
                    ))),
                    _ => {
                        return Err(ParseError {
                            msg: format!(
                                "Failed to parse value for SETPENCOLOR: {:?}",
                                tokens[curr_pos]
                            ),
                        });
                    }
                }
            }
            "TURN" => {
                curr_pos += 1;
                let expr = match_parse(&tokens, curr_pos, variables)?;

                // Because all expressions are parsed as floats, we need to convert
                // the float to an i32 for the TURN command.
                match expr {
                    Expression::Float(val) => ast.push(ASTNode::Command(Command::Turn(
                        Expression::Number(val as i32),
                    ))),
                    Expression::Variable(var) => {
                        ast.push(ASTNode::Command(Command::Turn(Expression::Variable(var))))
                    }
                    Expression::Query(query) => {
                        ast.push(ASTNode::Command(Command::Turn(Expression::Query(query))))
                    }
                    _ => {
                        return Err(ParseError {
                            msg: format!("Failed to parse value for TURN: {:?}", tokens[curr_pos]),
                        });
                    }
                }
            }
            "MAKE" => {
                curr_pos += 1;
                let var_name = tokens[curr_pos].trim_start_matches('"');

                curr_pos += 1;
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
                curr_pos += 1;
                if !tokens[curr_pos].starts_with('"') {
                    return Err(ParseError {
                        msg: format!("Invalid expression for ADDASSIGN: {:?}\nExpressions for ADDASSIGN should start with \"", tokens[curr_pos]),
                    });
                }
                let var_name = tokens[curr_pos].trim_start_matches('"');

                if !variables.contains_key(var_name) {
                    return Err(ParseError {
                        msg: format!("Variable not found for ADDASSIGN: {:?}", var_name),
                    });
                }

                curr_pos += 1;
                let expr = match_parse(&tokens, curr_pos, variables)?;

                ast.push(ASTNode::Command(Command::AddAssign(
                    var_name.to_string(),
                    expr,
                )));
            }
            "IF" => {
                curr_pos += 1; // Skip the IF token
                let condition = parse_conditions(&tokens, &mut curr_pos, variables)?;
                let block = parse_conditional_blocks(&tokens, &mut curr_pos, variables)?;
                ast.push(ASTNode::ControlFlow(ControlFlow::If { condition, block }));
            }
            "WHILE" => {
                curr_pos += 1; // Skip the WHILE token
                let condition = parse_conditions(&tokens, &mut curr_pos, variables)?;
                let block = parse_conditional_blocks(&tokens, &mut curr_pos, variables)?;
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
                    msg: format!("Failed to parse expression: {:?}", tokens[curr_pos]),
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

/// Matches up the token to its corresponding parsing function.
///
/// This is necessary because the token may be a variable, a query or a value which
/// all need to be parsed differently.
///
/// # Example
///
/// ```rust
/// use std::collections::HashMap;
///
/// let tokens = vec!["\"100"];
/// let expr = match_parse(&tokens, 0, &HashMap::new());
///
/// assert_eq!(expr, Ok(Expression::Float(100.0)));
/// ```
fn match_parse(
    tokens: &[&str],
    pos: usize,
    variables: &HashMap<String, Expression>,
) -> Result<Expression, ParseError> {
    if tokens[pos].starts_with('"') {
        parse_expression(tokens, pos)
            .map(Expression::Float)
            .or_else(|_| parse_bool(tokens, pos).map(Expression::Bool))
    } else if tokens[pos].starts_with(':') {
        let token = tokens[pos].trim_start_matches(':');
        if variables.contains_key(token) {
            Ok(Expression::Variable(token.to_string()))
        } else {
            Err(ParseError {
                msg: format!(
                    "Variable not found: {:?}, you may have forgotten to MAKE it.",
                    tokens[pos]
                ),
            })
        }
    } else {
        parse_query(tokens, pos).map(Expression::Query)
    }
}

fn parse_bool(tokens: &[&str], pos: usize) -> Result<bool, ParseError> {
    // Check that we cannot parse it into a float
    if tokens[pos].starts_with('"') {
        let val = tokens[pos].trim_start_matches('"');
        match val {
            "TRUE" => Ok(true),
            "FALSE" => Ok(false),
            _ => Err(ParseError {
                msg: format!("Failed to parse boolean: {:?}", tokens[pos]),
            }),
        }
    } else {
        Err(ParseError {
            msg: format!("Failed to parse boolean: {:?}", tokens[pos]),
        })
    }
}

/// Parse an expression from a token.
///
/// This expression defaults to a f32 value.
///
/// # Example
///
/// ```rust
/// let tokens = vec!["\"100"];
/// let expr = parse_expression(&tokens, 0);
///
/// assert_eq!(expr, Ok(100.0));
/// ```
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
            msg: format!("Cannot parse an invalid expression: {:?}", tokens[pos]),
        })
    }
}

/// Parse a query from a token.
/// A query is a special type of expression that returns a value from the turtle state.
///
/// # Example
///
/// ```rust
/// let tokens = vec!["XCOR"];
/// let query = parse_query(&tokens, 0);
///
/// assert_eq!(query, Ok(Query::XCor));
/// ```
fn parse_query(tokens: &[&str], pos: usize) -> Result<Query, ParseError> {
    let query = match tokens[pos] {
        "XCOR" => Query::XCor,
        "YCOR" => Query::YCor,
        "HEADING" => Query::Heading,
        "COLOR" => Query::Color,
        _ => {
            return Err(ParseError {
                msg: format!("Failed to parse this query expression: {:?}", tokens[pos]),
            });
        }
    };
    Ok(query)
}

/// Parse the conditions and expressions for the control flow statements.
///
/// # Example
/// ```rust
/// use std::collections::HashMap;
/// let mut variables: HashMap<String, Expression> = HashMap::new();
/// let tokens = vec!["EQ", "\"100", "\"100"];
///
/// let condition = parse_conditions(&tokens, &mut 0, &variables);
///
/// assert_eq!(condition, Ok(Condition::Equals(Expression::Float(100.0), Expression::Float(100.0))));
/// ```
fn parse_conditions(
    tokens: &[&str],
    curr_pos: &mut usize,
    variables: &HashMap<String, Expression>,
) -> Result<Condition, ParseError> {
    // Conditions will usually be in the form of:
    // <operator> <expression> <expression>

    let condition_idx = *curr_pos;

    *curr_pos += 1;
    let expr_1 = match_parse(tokens, *curr_pos, variables)?;

    *curr_pos += 1;
    let expr_2 = match_parse(tokens, *curr_pos, variables)?;

    *curr_pos += 1;
    let condition = match tokens[condition_idx] {
        "EQ" => Condition::Equals(expr_1, expr_2),
        "LT" => Condition::LessThan(expr_1, expr_2),
        "GT" => Condition::GreaterThan(expr_1, expr_2),
        _ => {
            return Err(ParseError {
                msg: format!("Invalid condition operator: {:?}", tokens[condition_idx]),
            });
        }
    };

    Ok(condition)
}

/// Parses the conditional blocks for the control flow statements (IF/WHILE)
/// into a vector of ASTNodes.
///
/// # Example
/// ```rust
/// use std::collections::HashMap;
/// let mut variables: HashMap<String, Expression> = HashMap::new();
///
/// let tokens = vec!["[", "PENDOWN", "FORWARD", "\"100", "]"];
/// let mut curr_pos = 0;
///
/// let block = parse_conditional_blocks(&tokens, &mut curr_pos, &mut variables);
/// assert_eq!(block, Ok(vec![ASTNode::Command(Command::PenDown),
///        ASTNode::Command(Command::Forward(Expression::Float(100.0)))]));
/// ```
fn parse_conditional_blocks(
    tokens: &[&str],
    curr_pos: &mut usize,
    variables: &mut HashMap<String, Expression>,
) -> Result<Vec<ASTNode>, ParseError> {
    if tokens[*curr_pos] != "[" {
        return Err(ParseError {
            msg: format!(
                "Expected start of a conditional block: {:?}",
                tokens[*curr_pos]
            ),
        });
    }
    *curr_pos += 1; // skipping '['

    let mut block: Vec<ASTNode> = Vec::new();

    let ast = parse_tokens(tokens[*curr_pos..].to_vec(), variables)?;
    block.extend(ast);

    // NOTE: Hack to get curr_pos to end of the conditional block since
    // parse_tokens does not direcly update curr_pos.
    // TODO: Refactor parse_tokens to include curr_pos as a parameter.
    while tokens[*curr_pos] != "]" && *curr_pos < tokens.len() {
        *curr_pos += 1;
    }

    if tokens[*curr_pos] != "]" {
        return Err(ParseError {
            msg: format!("Failed to parse conditional block: {:?}", tokens[*curr_pos]),
        });
    }

    Ok(block)
}
