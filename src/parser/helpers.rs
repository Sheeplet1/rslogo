//! Parsing helper functions.
//!
//! Contains the bulk of the parsing functionality and logic.

use std::collections::HashMap;

use super::{
    ast::{ASTNode, Condition, Expression, Math, Query},
    errors::ParseError,
    errors::ParseErrorKind::{self, VariableNotFound},
    parser::parse_tokens,
};

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
pub fn match_parse(
    tokens: &[&str],
    pos: &mut usize,
    vars: &mut HashMap<String, Expression>,
) -> Result<Expression, ParseError> {
    if tokens[*pos].starts_with('"') {
        // Normal expressions
        parse_expression(tokens, *pos).map(Expression::Float)
    } else if tokens[*pos].starts_with(':') {
        // Variables
        let token = tokens[*pos].trim_start_matches(':');
        if vars.contains_key(token) {
            Ok(Expression::Variable(token.to_string()))
        } else {
            // TODO: Add col & line
            Err(ParseError {
                kind: VariableNotFound {
                    var: token.to_string(),
                },
                col: None,
                line: None,
            })
        }
    } else if matches!(
        tokens[*pos],
        "+" | "-" | "*" | "/" | "EQ" | "LT" | "GT" | "NE" | "AND" | "OR"
    ) {
        parse_maths(tokens, pos, vars)
    } else {
        parse_query(tokens, *pos).map(Expression::Query)
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
pub fn parse_expression(tokens: &[&str], pos: usize) -> Result<f32, ParseError> {
    if tokens[pos].starts_with('"') {
        let token = tokens[pos].trim_start_matches('"');
        if token == "TRUE" {
            Ok(1.0)
        } else if token == "FALSE" {
            Ok(0.0)
        } else {
            token.parse::<f32>().map_err(|_| ParseError {
                // TODO: Col & Line
                kind: ParseErrorKind::InvalidSyntax {
                    details: format!("Cannot parse this expression as a float: {:?}", token),
                },
                col: None,
                line: None,
            })
        }
    } else {
        // TODO: Col & Line
        return Err(ParseError {
            kind: ParseErrorKind::InvalidSyntax {
                details: format!("Cannot parse this expression as a float: {:?}", tokens[pos]),
            },
            col: None,
            line: None,
        });
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
pub fn parse_query(tokens: &[&str], pos: usize) -> Result<Query, ParseError> {
    let query = match tokens[pos] {
        "XCOR" => Query::XCor,
        "YCOR" => Query::YCor,
        "HEADING" => Query::Heading,
        "COLOR" => Query::Color,
        _ => {
            return Err(ParseError {
                // TODO: Line & Col
                kind: ParseErrorKind::InvalidSyntax {
                    details: format!("Could not parse this token as a query: {:?}", tokens[pos]),
                },
                col: None,
                line: None,
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
/// let mut vars: HashMap<String, Expression> = HashMap::new();
/// let tokens = vec!["EQ", "\"100", "\"100"];
///
/// let condition = parse_conditions(&tokens, &mut 0, &vars);
///
/// assert_eq!(condition, Ok(Condition::Equals(Expression::Float(100.0), Expression::Float(100.0))));
/// ```
pub fn parse_conditions(
    tokens: &[&str],
    curr_pos: &mut usize,
    vars: &mut HashMap<String, Expression>,
) -> Result<Condition, ParseError> {
    let condition_idx = *curr_pos;

    // If condition_idx is not an condition but a boolean, we return early.
    if !matches!(tokens[condition_idx], "EQ" | "LT" | "GT" | "AND" | "OR") {
        let res = match_parse(tokens, curr_pos, vars)
            .map(|expr| Condition::Equals(expr, Expression::Float(1.0)));
        *curr_pos += 1;
        return res;
    }

    *curr_pos += 1;
    let expr_1 = match_parse(tokens, curr_pos, vars)?;

    *curr_pos += 1;
    let expr_2 = match_parse(tokens, curr_pos, vars)?;

    *curr_pos += 1;
    let condition = match tokens[condition_idx] {
        "EQ" => Condition::Equals(expr_1, expr_2),
        "LT" => Condition::LessThan(expr_1, expr_2),
        "GT" => Condition::GreaterThan(expr_1, expr_2),
        "AND" => Condition::And(expr_1, expr_2),
        "OR" => Condition::Or(expr_1, expr_2),
        _ => {
            return Err(ParseError {
                kind: ParseErrorKind::InvalidSyntax {
                    details: format!("Invalid condition provided: {:?}", tokens[condition_idx]),
                },
                col: None,
                line: None,
            })
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
/// let mut vars: HashMap<String, Expression> = HashMap::new();
///
/// let tokens = vec!["[", "PENDOWN", "FORWARD", "\"100", "]"];
/// let mut curr_pos = 0;
///
/// let block = parse_conditional_blocks(&tokens, &mut curr_pos, &mut vars);
/// assert_eq!(block, Ok(vec![ASTNode::Command(Command::PenDown),
///        ASTNode::Command(Command::Forward(Expression::Float(100.0)))]));
/// ```
pub fn parse_conditional_blocks(
    tokens: &[&str],
    curr_pos: &mut usize,
    vars: &mut HashMap<String, Expression>,
) -> Result<Vec<ASTNode>, ParseError> {
    if tokens[*curr_pos] != "[" {
        return Err(ParseError {
            kind: ParseErrorKind::InvalidSyntax {
                details: format!(
                    "Expected the start of a conditiona block: '[', found: {:?}",
                    tokens[*curr_pos]
                ),
            },
            col: None,
            line: None,
        });
    }
    *curr_pos += 1; // skipping '['

    let mut block: Vec<ASTNode> = Vec::new();

    while *curr_pos < tokens.len() && tokens[*curr_pos] != "]" {
        let ast = parse_tokens(tokens.to_vec(), curr_pos, vars)?;
        block.extend(ast);
    }

    if tokens[*curr_pos] != "]" {
        return Err(ParseError {
            kind: ParseErrorKind::InvalidSyntax {
                details: format!(
                    "Expected the end of a conditional block: ']', found: {:?}",
                    tokens[*curr_pos]
                ),
            },
            col: None,
            line: None,
        });
    }

    Ok(block)
}

/// Parse mathematical expressions. Includes both basic and logical arithmetics.
///
/// # Example
/// ```rust
/// use std::collections::HashMap;
/// let mut vars: HashMap<String, Expression> = HashMap::new();
/// let tokens = vec!["+", "\"100", "\"100"];
/// let mut curr_pos = 0;
/// let expr = parse_maths(&tokens, &mut curr_pos, &mut vars);
/// assert_eq!(expr, Ok(Expression::Math(Box::new(Math::Add(Expression::Float(100.0), Expression::Float(100.0)))));
/// ```
pub fn parse_maths(
    tokens: &[&str],
    curr_pos: &mut usize,
    vars: &mut HashMap<String, Expression>,
) -> Result<Expression, ParseError> {
    // Maths will usually be in the form of: <operator> <expression> <expression>
    // operators will be +, -, *, /
    let operator = tokens[*curr_pos];
    let res = match operator {
        "+" | "-" | "*" | "/" | "EQ" | "LT" | "GT" | "NE" | "AND" | "OR" => {
            *curr_pos += 1;
            let expr_1 = match_parse(tokens, curr_pos, vars)?;
            *curr_pos += 1;
            let expr_2 = match_parse(tokens, curr_pos, vars)?;

            match operator {
                "+" => Expression::Math(Box::new(Math::Add(expr_1, expr_2))),
                "-" => Expression::Math(Box::new(Math::Sub(expr_1, expr_2))),
                "*" => Expression::Math(Box::new(Math::Mul(expr_1, expr_2))),
                "/" => Expression::Math(Box::new(Math::Div(expr_1, expr_2))),
                "EQ" => Expression::Math(Box::new(Math::Eq(expr_1, expr_2))),
                "LT" => Expression::Math(Box::new(Math::Lt(expr_1, expr_2))),
                "GT" => Expression::Math(Box::new(Math::Gt(expr_1, expr_2))),
                "NE" => Expression::Math(Box::new(Math::Ne(expr_1, expr_2))),
                "AND" => Expression::Math(Box::new(Math::And(expr_1, expr_2))),
                "OR" => Expression::Math(Box::new(Math::Or(expr_1, expr_2))),
            }
        }
        _ => {
            return Err(ParseError {
                kind: ParseErrorKind::InvalidSyntax {
                    details: format!("Invalid operator provided: {:?}", operator),
                },
                col: None,
                line: None,
            })
        }
    };

    Ok(res)
}
