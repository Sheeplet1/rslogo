# TODO

## Current

```

TO Box
   FORWARD "10
   LEFT "10
   BACK "10
   RIGHT "10
END

PENDOWN

MAKE "BOXLINE "2

WHILE LT :BOXLINE "60 [
      TURN "20

      SETPENCOLOR + COLOR "2

      Box

      IF LT "8 COLOR [
         SETPENCOLOR "2
      ]

      ADDASSIGN "BOXLINE "2
]

AST: [
    Procedure {
        name: "Box",
        args: [],
        body: [
            Command(
                Forward(Float(10.0))
            ),
            Command(
                Left(Float(10.0))
            )
            Command(
                Back(Float(10.0))
            )
            Command(
                Right(Float(10.0))
            )
        ]
    },

    Command(
        Pendown
    )

    Command(
        Make(
            "BOXLINE",
            Float(2.0)
        )
    )

    ControlFlow(
        While {
            condition: LessThan(
                Variable("BOXLINE"),
                Float(60.0)
            ),
            block: [
                Command(
                    Turn(Float(20.0))
                )

                Command(
                    SetPenColor(
                        Add(
                            Query(COLOR),
                            Float(2.0)
                        )
                    )
                )

                Command(
                    ProcedureCall("Box", [])
                )

                ControlFlow(
                    If {
                        condition: LessThan(Float(8.0), Query(Color)),
                        block: [Command(SetPenColor(2.0))]
                    }
                )

                Command(AddAssign(Variable("BOXLINE"), Float(2.0)))
            ]
        }
    )
]
```

Key Words are `TO` and `END` to define a procedure.

```
TO <name> <"vec of args>
    Block of commands
END
```

We know arguments has ended once there is a command

## Backlog

- [ ] Make tests
- [ ] It might be worthwhile to explore into popping elements from tokens
      rather than iterating over tokens. This could reduce memory usage required
      over the run-time of the application.

# BUGS
