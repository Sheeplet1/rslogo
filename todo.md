# TODO

## Current

- [ ] IF Command

Problem here is the recursion in parsing the command block.
We need to keep track of our current position in the `tokens` vector.

Problem is with initialising AST in the `parse_tokens` function, I believe we
should move this outside of the scope, initialise it there and then pass it
in as a mutable parameter.

`parse_tokens` constructs an AST and returns it as a vector.
Initialise AST outside of scope
Pass in AST as a parameter
Fill out AST using

1. add `curr_pos` as a parameter to `parse_tokens`
2. Refactor `parse_tokens` return a `ASTNode` instead of a `Vec<ASTNode>`.

OR

Create a new function to specifically parse conditional blocks and return a
ASTNode that way.

- [ ] WHILE command

## Backlog

- [ ] Make tests

# BUGS
