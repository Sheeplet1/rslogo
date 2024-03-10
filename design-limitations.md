# Design Limitations

## Lack of USize expression

In my enum for `Expression`, I did not include USize. This was due to the way
I parsed my tokens and specifically for `SetPenColor`, I had to parse it
differently due to its parameter being of type `usize`.

Instead of converting it to a `usize` in my parser, I decided to cast it as
`usize` during the execution phase instead which resulted in less lines of code
and a simpler execution.

However, if more `usize` parameters were added down the line, it would be helpful
to readd it to `Expression`.

See the parsing code I had for `SetPenColor` before below:

```
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

```
