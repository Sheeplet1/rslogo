# TODO

## Current

```5_00
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

let ast = vec![
    // Procedure Definition: Box
    ASTNode::ProcedureDefinition {
        name: "Box".to_string(),
        parameters: vec![],
        body: vec![
            ASTNode::Command(Command::Forward(Expression::Float(10.0))),
            ASTNode::Command(Command::Left(Expression::Float(10.0))),
            ASTNode::Command(Command::Back(Expression::Float(10.0))),
            ASTNode::Command(Command::Right(Expression::Float(10.0))),
        ],
    },

    // PenDown
    ASTNode::Command(Command::PenDown),

    // Make "BOXLINE" "2"
    ASTNode::Command(Command::Make("BOXLINE".to_string(), Expression::Float(2.0))),

    // While Loop
    ASTNode::ControlFlow(ControlFlow::While {

        condition: Condition::LessThan(
            Expression::Variable("BOXLINE".to_string()),
            Expression::Float(60.0),
        ),

        block: vec![
            ASTNode::Command(Command::Turn(Expression::Float(20.0))),

            ASTNode::Command(Command::SetPenColor(Expression::Math(Box::new(
                Math::Add(
                    Expression::Query(Query::Color),
                    Expression::Float(2.0),
                )
            )))),

            // Procedure Call: Box
            ASTNode::ProcedureCall {
                name: "Box".to_string(),
                arguments: vec![],
            },

            // If Statement
            ASTNode::ControlFlow(ControlFlow::If {
                condition: Condition::LessThan(
                    Expression::Float(8.0),
                    Expression::Query(Query::Color),
                ),
                block: vec![
                    ASTNode::Command(Command::SetPenColor(Expression::Float(2.0))),
                ],
            }),

            // AddAssign "BOXLINE" "2"
            ASTNode::Command(Command::AddAssign("BOXLINE".to_string(), Expression::Float(2.0))),
        ],
    }),
];

```

## Backlog

- [ ] Make tests
- [ ] It might be worthwhile to explore into popping elements from tokens
      rather than iterating over tokens. This could reduce memory usage required
      over the run-time of the application.

# BUGS
