use std::collections::HashMap;

use crate::errors::ParseError;

use super::{
    ast::{ASTNode, Condition, Expression, Query},
    maths::parse_maths,
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
    variables: &mut HashMap<String, Expression>,
) -> Result<Expression, ParseError> {
    if tokens[*pos].starts_with('"') {
        parse_expression(tokens, *pos).map(Expression::Float)
    } else if tokens[*pos].starts_with(':') {
        let token = tokens[*pos].trim_start_matches(':');
        if variables.contains_key(token) {
            Ok(Expression::Variable(token.to_string()))
        } else {
            Err(ParseError {
                msg: format!(
                    "Variable not found: {:?}, you may have forgotten to MAKE it.",
                    tokens[*pos]
                ),
            })
        }
    } else if tokens[*pos] == "+"
        || tokens[*pos] == "-"
        || tokens[*pos] == "*"
        || tokens[*pos] == "/"
    {
        let res = parse_maths(tokens, pos, variables).map(Expression::Float);
        println!("{:?}", res);
        res
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
                msg: format!("Failed to parse expression: {:?}", tokens[pos]),
            })
        }
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
pub fn parse_query(tokens: &[&str], pos: usize) -> Result<Query, ParseError> {
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
pub fn parse_conditions(
    tokens: &[&str],
    curr_pos: &mut usize,
    variables: &mut HashMap<String, Expression>,
) -> Result<Condition, ParseError> {
    // Conditions will usually be in the form of:
    // <operator> <expression> <expression>

    let condition_idx = *curr_pos;

    *curr_pos += 1;
    let expr_1 = match_parse(tokens, curr_pos, variables)?;

    *curr_pos += 1;
    let expr_2 = match_parse(tokens, curr_pos, variables)?;

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
pub fn parse_conditional_blocks(
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

    while *curr_pos < tokens.len() && tokens[*curr_pos] != "]" {
        let ast = parse_tokens(tokens.to_vec(), curr_pos, variables)?;
        block.extend(ast);
    }

    if tokens[*curr_pos] != "]" {
        return Err(ParseError {
            msg: format!("Failed to parse conditional block: {:?}", tokens[*curr_pos]),
        });
    }

    Ok(block)
}
