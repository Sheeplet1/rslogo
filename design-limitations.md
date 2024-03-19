# Design Limitations

## Defaulting to f32 for execution and typecasting to other types when required

My expressions default to f32 values before being typecasted for the appropriate
functions.

For example, `SetPenColor` requires `usize`, before execution, this value is
parsed as an `f32` before being typecasted to `usize`.

Limitations of this is:

1. Loss of precision and information between conversions which can cause
   unexpected behaviours such as overflow, underflow, or rounding errors.
2. Goes against Rust's type safety. My reasoning for this was to reduce
   complexity of the code to handle different enums, however, this could also
   be an extension of my design choices.
3. Reduced clarity and limitations on extending functionality on other types
   such as `i32` since they would have to be typecasted too.

## Lack of Bools

My design does not implement booleans properly, instead, it evaluates booleans
into their corresponding float values, `1.0` for `TRUE`, and `0.0` for `FALSE`.

This is quite hacky but implemented this way to reduce complexity of the code due
to time limitations and previous design choices.

Previous design choices being to default all `Expression`s to `f32`.

## Multiple And/Or enums

As of right now, I believe them to be doing similar things but they have been
separated as a consequence of design.

I think having multiple And/Or enums has made it confusing and can interfere
with scalability.
