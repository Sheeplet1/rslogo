# TODO

## Current

- [ ] WHILE command

- [ ] Refactor `parse_token` to include curr_pos as an argument. This is
      necessary for recursive functions.

## Backlog

- [ ] Make tests

# BUGS

Nested IF Statements are causing parsing errors which is bypassing
AddAssign in 3_06.

Parsing is wrong for nested IF Statements.

THIS is becasue curr_pos is not updating correctly for nested if statements.
Need to add curr_pos as an argument to the parse_tokens so that it can be
recursively tracked.

# Design Limitations

Booleans are proving to be quite hard to implement at the moment.

It might be easier to convert them into float values and evaluate that way for now.

Obviously, this introduces a lot of bugs since there are many cases where the values
can be 1.0 and 0.0 which would interfere with function.

Otherwise, I will need to rethink through the design to reimplement Bools

OR

We add as a design limitation that bools are evaluated to 1.0 and 0.0 which
can introduce bugs.
