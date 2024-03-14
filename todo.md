# TODO

## Current

- [ ] IF Command

Create a new function to specifically parse conditional blocks and return a
ASTNode that way.

The function will take a start and end index for `tokens` which indicate
the start and end of the block.

We will parse through that block using `parse_tokens`, but ensure to let
the result be equal to a defined `block` variable.

Now that `block` will be a vector, which we want according to how we defined
the enums.

So we can just return that and done! make sure to set curr_pos to be equal to
the end index + 1.

Execute should be relatively easy. Just keep track of the values of both
expressions in the condition and increment/decrement accordingly. End the loop
once done and move on.

- [ ] WHILE command

If we can refactor this into the IF execution block, then that would be good.

Otherwise, logic wise:

1. Get the two values that are being compared.
2. Need a way to continously evaluate the conditions... which means in each loop
   these values need to be updated.

The problem is that with comparing each value, we are giving away ownership but also
in that the values are `Expression`s which means we need to `parse_expression` on them
which also gives away ownership of `variables` and `turtle`.

We either need to redesign or hack our way around this.

## Backlog

- [ ] Make tests

# BUGS
