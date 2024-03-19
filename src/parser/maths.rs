use std::collections::HashMap;

use crate::{
    errors::ParseError,
    parser::{ast::Math, helpers::match_parse},
};

use super::ast::Expression;

pub fn parse_maths(
    tokens: &[&str],
    curr_pos: &mut usize,
    variables: &mut HashMap<String, Expression>,
) -> Result<Expression, ParseError> {
    // Maths will usually be in the form of: <operator> <expression> <expression>
    // operators will be +, -, *, /
    let res = match tokens[*curr_pos] {
        "+" | "-" | "*" | "/" | "EQ" | "LT" | "GT" | "NE" | "AND" | "OR" => {
            let operator = tokens[*curr_pos];

            *curr_pos += 1;
            let expr_1 = match_parse(tokens, curr_pos, variables)?;
            *curr_pos += 1;
            let expr_2 = match_parse(tokens, curr_pos, variables)?;

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
                _ => {
                    return Err(ParseError {
                        msg: format!("Invalid operator: {:?}", operator),
                    })
                }
            }
        }

        _ => {
            return Err(ParseError {
                msg: format!("Invalid operator: {:?}", tokens[*curr_pos]),
            })
        }
    };

    Ok(res)
}
