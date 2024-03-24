//! Parsing helper functions.
//!
//! Contains the bulk of the parsing functionality and how each `Expression`
//! is parsed.

use std::collections::HashMap;

use super::{
    ast::{ASTNode, Condition, Expression, Math, Query},
    errors::ParseError,
    errors::ParseErrorKind::{self, VariableNotFound},
    parser::parse_tokens,
};

/// Matches and parses a token into an `Expression`.
///
/// # Example
///
/// ```rust
/// use std::collections::HashMap;
///
/// let tokens = vec!["\"100"];
/// let expr = match_parse(&tokens, &mut 0, &mut HashMap::new())?;
///
/// assert_eq!(expr, Expression::Float(100.0));
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
            Err(ParseError {
                kind: VariableNotFound {
                    var: token.to_string(),
                },
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
/// let expr = parse_expression(&tokens, 0)?;
///
/// assert_eq!(expr, 100.0);
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
                kind: ParseErrorKind::InvalidSyntax {
                    msg: format!("Cannot parse this expression as a float: {:?}", token),
                },
            })
        }
    } else {
        Err(ParseError {
            kind: ParseErrorKind::InvalidSyntax {
                msg: format!("Cannot parse this expression as a float: {:?}", tokens[pos]),
            },
        })
    }
}

/// Parse a query from a token.
///
/// A query returns msg specific to the turtle's state.
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
                kind: ParseErrorKind::InvalidSyntax {
                    msg: format!("Could not parse this token as a query: {:?}", tokens[pos]),
                },
            });
        }
    };
    Ok(query)
}

/// Parse the conditions for the control flow statements (IF/WHILE).
///
/// # Example
///
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

    // If condition_idx is not an condition but a boolean, we parse the
    // boolean as a condition and return early.
    if !matches!(tokens[condition_idx], "EQ" | "LT" | "GT" | "AND" | "OR") {
        let res = match_parse(tokens, curr_pos, vars)
            .map(|expr| Condition::Equals(expr, Expression::Float(1.0)));
        *curr_pos += 1;
        return res;
    }

    // Otherwise, we parse the condition as normal.
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
        _ => unreachable!(),
    };

    Ok(condition)
}

/// Parses the blocks of code for the control flow statements (IF/WHILE)
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
                msg: format!(
                    "Expected the start of a conditiona block: '[', found: {:?}",
                    tokens[*curr_pos]
                ),
            },
        });
    }
    *curr_pos += 1; // skipping '['

    let mut block: Vec<ASTNode> = Vec::new();

    while *curr_pos < tokens.len() && tokens[*curr_pos] != "]" {
        let ast = parse_tokens(tokens.to_vec(), curr_pos, vars)?;
        block.extend(ast);
    }

    // If we reach the end of the tokens and the block hasn't been closed yet,
    // we return an error.
    if *curr_pos >= tokens.len() || tokens[*curr_pos] != "]" {
        return Err(ParseError {
            kind: ParseErrorKind::InvalidSyntax {
                msg: "Expected the end of a conditional block: ']'".to_string(),
            },
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
    // operators will be +, -, *, /, "EQ", "LT", "GT", "NE", "AND", "OR".
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
                _ => unreachable!(),
            }
        }
        _ => {
            return Err(ParseError {
                kind: ParseErrorKind::InvalidSyntax {
                    msg: format!("Invalid operator provided: {:?}", operator),
                },
            })
        }
    };

    Ok(res)
}

#[cfg(test)]
mod tests {
    use crate::parser::ast::Command;

    use super::*;

    #[test]
    fn test_parse_float_expr() {
        let tokens = vec!["\"100"];
        let expr = parse_expression(&tokens, 0).unwrap();

        assert_eq!(expr, 100.0);
    }

    #[test]
    fn test_parse_true_expr() {
        let tokens = vec!["\"TRUE"];
        let expr = parse_expression(&tokens, 0).unwrap();

        assert_eq!(expr, 1.0);
    }

    #[test]
    fn test_parse_false_expr() {
        let tokens = vec!["\"FALSE"];
        let expr = parse_expression(&tokens, 0).unwrap();

        assert_eq!(expr, 0.0);
    }

    #[test]
    fn test_invalid_parse_expr() {
        let tokens = vec!["TOKEN"];
        let expr = parse_expression(&tokens, 0);

        assert!(expr.is_err());
    }

    #[test]
    fn test_invalid_parse_expr_2() {
        let tokens = vec!["\"TOKEN"];
        let expr = parse_expression(&tokens, 0);

        assert!(expr.is_err());
    }

    #[test]
    fn test_parse_query() {
        let tokens = vec!["XCOR"];
        let query = parse_query(&tokens, 0).unwrap();

        assert_eq!(query, Query::XCor);
    }

    #[test]
    fn test_parse_conditions() {
        let mut vars: HashMap<String, Expression> = HashMap::new();
        let tokens = vec!["EQ", "\"100", "\"100"];

        let condition = parse_conditions(&tokens, &mut 0, &mut vars).unwrap();

        assert_eq!(
            condition,
            Condition::Equals(Expression::Float(100.0), Expression::Float(100.0))
        );
    }

    #[test]
    fn test_parse_condition_bool() {
        let mut vars: HashMap<String, Expression> = HashMap::new();
        vars.insert("x".to_string(), Expression::Float(1.0));

        let tokens = vec![":x"];
        let condition = parse_conditions(&tokens, &mut 0, &mut vars).unwrap();

        assert_eq!(
            condition,
            Condition::Equals(
                Expression::Variable("x".to_string()),
                Expression::Float(1.0)
            )
        );
    }

    #[test]
    fn test_parse_conditions_lt() {
        let mut vars: HashMap<String, Expression> = HashMap::new();
        let tokens = vec!["LT", "\"80", "\"100"];

        let condition = parse_conditions(&tokens, &mut 0, &mut vars).unwrap();

        assert_eq!(
            condition,
            Condition::LessThan(Expression::Float(80.0), Expression::Float(100.0))
        );
    }

    #[test]
    fn test_parse_conditions_gt() {
        let mut vars: HashMap<String, Expression> = HashMap::new();
        let tokens = vec!["GT", "\"100", "\"80"];

        let condition = parse_conditions(&tokens, &mut 0, &mut vars).unwrap();

        assert_eq!(
            condition,
            Condition::GreaterThan(Expression::Float(100.0), Expression::Float(80.0))
        );
    }

    #[test]
    fn test_parse_conditions_and() {
        let mut vars: HashMap<String, Expression> = HashMap::new();
        let tokens = vec!["AND", "\"100", "\"100"];

        let condition = parse_conditions(&tokens, &mut 0, &mut vars).unwrap();

        assert_eq!(
            condition,
            Condition::And(Expression::Float(100.0), Expression::Float(100.0))
        );
    }

    #[test]
    fn test_parse_conditions_or() {
        let mut vars: HashMap<String, Expression> = HashMap::new();
        let tokens = vec!["OR", "\"100", "\"100"];

        let condition = parse_conditions(&tokens, &mut 0, &mut vars).unwrap();

        assert_eq!(
            condition,
            Condition::Or(Expression::Float(100.0), Expression::Float(100.0))
        );
    }

    #[test]
    fn test_parse_invalid_cond() {
        let mut vars: HashMap<String, Expression> = HashMap::new();
        let tokens = vec!["INVALID", "\"100", "\"100"];

        let condition = parse_conditions(&tokens, &mut 0, &mut vars);

        assert!(condition.is_err());
    }

    #[test]
    fn test_parse_conditional_blocks() {
        let mut vars: HashMap<String, Expression> = HashMap::new();

        let tokens = vec!["[", "PENDOWN", "FORWARD", "\"100", "]"];
        let mut curr_pos = 0;

        let block = parse_conditional_blocks(&tokens, &mut curr_pos, &mut vars).unwrap();
        assert_eq!(
            block,
            vec![
                ASTNode::Command(Command::PenDown),
                ASTNode::Command(Command::Forward(Expression::Float(100.0)))
            ]
        );
    }

    #[test]
    fn test_parse_cond_block_inval_start() {
        let mut vars: HashMap<String, Expression> = HashMap::new();

        let tokens = vec!["PENDOWN", "FORWARD", "\"100", "]"];
        let mut curr_pos = 0;

        let block = parse_conditional_blocks(&tokens, &mut curr_pos, &mut vars);

        assert!(block.is_err());
    }

    #[test]
    fn test_parse_cond_block_inval_end() {
        let mut vars: HashMap<String, Expression> = HashMap::new();

        let tokens = vec!["[", "PENDOWN", "FORWARD", "\"100"];
        let mut curr_pos = 0;

        let block = parse_conditional_blocks(&tokens, &mut curr_pos, &mut vars);

        assert!(block.is_err());
    }

    #[test]
    fn test_parse_maths_add() {
        let mut vars: HashMap<String, Expression> = HashMap::new();
        let tokens = vec!["+", "\"100", "\"100"];
        let mut curr_pos = 0;
        let expr = parse_maths(&tokens, &mut curr_pos, &mut vars).unwrap();
        assert_eq!(
            expr,
            Expression::Math(Box::new(Math::Add(
                Expression::Float(100.0),
                Expression::Float(100.0)
            )))
        );
    }

    #[test]
    fn test_parse_maths_sub() {
        let mut vars: HashMap<String, Expression> = HashMap::new();
        let tokens = vec!["-", "\"100", "\"100"];
        let mut curr_pos = 0;
        let expr = parse_maths(&tokens, &mut curr_pos, &mut vars).unwrap();
        assert_eq!(
            expr,
            Expression::Math(Box::new(Math::Sub(
                Expression::Float(100.0),
                Expression::Float(100.0)
            )))
        );
    }

    #[test]
    fn test_parse_maths_mul() {
        let mut vars: HashMap<String, Expression> = HashMap::new();
        let tokens = vec!["*", "\"100", "\"100"];
        let mut curr_pos = 0;
        let expr = parse_maths(&tokens, &mut curr_pos, &mut vars).unwrap();
        assert_eq!(
            expr,
            Expression::Math(Box::new(Math::Mul(
                Expression::Float(100.0),
                Expression::Float(100.0)
            )))
        );
    }

    #[test]
    fn test_parse_maths_div() {
        let mut vars: HashMap<String, Expression> = HashMap::new();
        let tokens = vec!["/", "\"100", "\"100"];
        let mut curr_pos = 0;
        let expr = parse_maths(&tokens, &mut curr_pos, &mut vars).unwrap();
        assert_eq!(
            expr,
            Expression::Math(Box::new(Math::Div(
                Expression::Float(100.0),
                Expression::Float(100.0)
            )))
        );
    }

    #[test]
    fn test_parse_maths_eq() {
        let mut vars: HashMap<String, Expression> = HashMap::new();
        let tokens = vec!["EQ", "\"100", "\"100"];
        let mut curr_pos = 0;
        let expr = parse_maths(&tokens, &mut curr_pos, &mut vars).unwrap();
        assert_eq!(
            expr,
            Expression::Math(Box::new(Math::Eq(
                Expression::Float(100.0),
                Expression::Float(100.0)
            )))
        );
    }

    #[test]
    fn test_parse_maths_lt() {
        let mut vars: HashMap<String, Expression> = HashMap::new();
        let tokens = vec!["LT", "\"100", "\"100"];
        let mut curr_pos = 0;
        let expr = parse_maths(&tokens, &mut curr_pos, &mut vars).unwrap();
        assert_eq!(
            expr,
            Expression::Math(Box::new(Math::Lt(
                Expression::Float(100.0),
                Expression::Float(100.0)
            )))
        );
    }

    #[test]
    fn test_parse_maths_gt() {
        let mut vars: HashMap<String, Expression> = HashMap::new();
        let tokens = vec!["GT", "\"100", "\"100"];
        let mut curr_pos = 0;
        let expr = parse_maths(&tokens, &mut curr_pos, &mut vars).unwrap();
        assert_eq!(
            expr,
            Expression::Math(Box::new(Math::Gt(
                Expression::Float(100.0),
                Expression::Float(100.0)
            )))
        );
    }

    #[test]
    fn test_parse_maths_ne() {
        let mut vars: HashMap<String, Expression> = HashMap::new();
        let tokens = vec!["NE", "\"100", "\"100"];
        let mut curr_pos = 0;
        let expr = parse_maths(&tokens, &mut curr_pos, &mut vars).unwrap();
        assert_eq!(
            expr,
            Expression::Math(Box::new(Math::Ne(
                Expression::Float(100.0),
                Expression::Float(100.0)
            )))
        );
    }

    #[test]
    fn test_parse_maths_and() {
        let mut vars: HashMap<String, Expression> = HashMap::new();
        let tokens = vec!["AND", "\"100", "\"100"];
        let mut curr_pos = 0;
        let expr = parse_maths(&tokens, &mut curr_pos, &mut vars).unwrap();
        assert_eq!(
            expr,
            Expression::Math(Box::new(Math::And(
                Expression::Float(100.0),
                Expression::Float(100.0)
            )))
        );
    }

    #[test]
    fn test_parse_maths_or() {
        let mut vars: HashMap<String, Expression> = HashMap::new();
        let tokens = vec!["OR", "\"100", "\"100"];
        let mut curr_pos = 0;
        let expr = parse_maths(&tokens, &mut curr_pos, &mut vars).unwrap();
        assert_eq!(
            expr,
            Expression::Math(Box::new(Math::Or(
                Expression::Float(100.0),
                Expression::Float(100.0)
            )))
        );
    }

    #[test]
    fn test_parse_maths_invalid_operator() {
        let mut vars: HashMap<String, Expression> = HashMap::new();
        let tokens = vec!["INVALID", "\"100", "\"100"];
        let mut curr_pos = 0;
        let expr = parse_maths(&tokens, &mut curr_pos, &mut vars);

        assert!(expr.is_err());
    }

    #[test]
    fn test_match_parse() {
        let mut vars: HashMap<String, Expression> = HashMap::new();
        let tokens = vec!["\"100"];
        let expr = match_parse(&tokens, &mut 0, &mut vars).unwrap();

        assert_eq!(expr, Expression::Float(100.0));
    }

    #[test]
    fn test_match_parse_variable() {
        let mut vars: HashMap<String, Expression> = HashMap::new();
        vars.insert("x".to_string(), Expression::Float(100.0));
        let tokens = vec![":x"];
        let expr = match_parse(&tokens, &mut 0, &mut vars).unwrap();

        assert_eq!(expr, Expression::Variable("x".to_string()));
    }

    #[test]
    fn test_match_parse_invalid_var() {
        let mut vars: HashMap<String, Expression> = HashMap::new();
        let tokens = vec![":x"];
        let expr = match_parse(&tokens, &mut 0, &mut vars);

        assert!(expr.is_err());
    }

    #[test]
    fn test_match_parse_maths() {
        let mut vars: HashMap<String, Expression> = HashMap::new();
        let tokens = vec!["+", "\"100", "\"100"];
        let mut curr_pos = 0;
        let expr = match_parse(&tokens, &mut curr_pos, &mut vars).unwrap();
        assert_eq!(
            expr,
            Expression::Math(Box::new(Math::Add(
                Expression::Float(100.0),
                Expression::Float(100.0)
            )))
        );
    }

    #[test]
    fn test_match_parse_query() {
        let mut vars: HashMap<String, Expression> = HashMap::new();
        let tokens = vec!["XCOR"];
        let query = match_parse(&tokens, &mut 0, &mut vars).unwrap();

        assert_eq!(query, Expression::Query(Query::XCor));
    }
}
