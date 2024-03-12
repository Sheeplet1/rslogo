# TODO

## Current

- [ ] Update documentation and provide examples

## Backlog

- [ ] Make tests

# BUGS

Parsing for `SETPENCOLOR`, `TURN`, `SETHEADING` need to be revamped
since `Expression::Variable` has been added. We have considered this as a
valid expression to be added to the AST.

Refactor it out.
