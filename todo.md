# TODO

## Current

- [ ] Parsing for mathematical expressions
  - [ ] Evaluation

```
If token is an operator (+, -, \*, /) {
    Lets append until end of mathematical expression into a vector
    Reverse the vector
    Evaluate via reverse polish notation
}

==============================================================================
PENDOWN
FORWARD + "5 "3

AST: [
    Command(
        Pendown
    ),

    Command(
        Forward(
            Addition(
                Float(5.0),
                Float(3.0)
            )
        )
    )
]

==============================================================================
PENDOWN
IF AND GT "4 "2 LT "3 "6 [
   FORWARD "10
]

AST: [
    Command(
        Pendown
    ),

    ControlFlow (
        If {
            condition: And(
                GreaterThan(
                    Float(4.0),
                    Float(2.0),
                ),
                LessThan(
                    Float(3.0),
                    Float(6.0)
                )
            )
            block: [
                Forward(
                    Float(10.0)
                )
            ]
        }
    )
]

==============================================================================
// ((3 < 3 + 1) && (9 > 8) || (8 / 2 < 8 / 3)
MAKE "x OR AND LT "3 + "3 "1 GT "9 "8 LT / "8 "2 / "8 "3

IF :x [
   PENDOWN
]

TURN "135
FORWARD "20
LEFT "100

AST: [
    Command(
        Make(
            "x",
            Or(
                And(
                    LessThan(
                        Float(3.0),
                        Addition(
                            Float(3.0)
                            Float(1.0)
                        )
                    ),
                    GreaterThan(
                        Float(9.0),
                        Float(8.0)
                    )
                ),
                LessThan(
                    Division(
                        Float(8.0),
                        Float(2.0)
                    ),
                    Division(
                        Float(8.0),
                        Float(3.0)
                    )
                )
            )
        ),
    ),

    ControlFlow(
        If {
            condition: Variable("x"),
            block: [
                Command(Pendown)
            ]
        }
    ),

    Command(
        Turn(
            Number(135)
        )
    ),

    Command(
        Forward(
            Float(20.0)
        )
    ),

    Command(
        Left(
            Float(100.0)
        )
    )
]

```

## Backlog

- [ ] Make tests
- [ ] It might be worthwhile to explore into popping elements from tokens
      rather than iterating over tokens. This could reduce memory usage required
      over the run-time of the application.

# BUGS
