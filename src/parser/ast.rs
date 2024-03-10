#[derive(Debug)]
pub enum ASTNode {
    Command(Command),
    // ControlFlow(ControlFlow),
}

#[derive(Debug, Clone)]
pub enum Expression {
    Float(f32),
    Number(i32),
    Query(Query),
}

#[derive(Debug, Clone)]
pub enum Command {
    Forward(Expression),
    Back(Expression),
    Left(Expression),
    Right(Expression),
    PenUp,
    PenDown,
    SetPenColor(Expression),
    Turn(Expression),
    SetHeading(Expression),
    SetX(Expression),
    SetY(Expression),
    Make(String, Expression),
}

#[derive(Debug, Clone)]
pub enum Query {
    XCor,
    YCor,
    Heading,
    Color,
}

// pub enum ControlFlow {
//     If {
//         condition: Condition,
//         block: Vec<ASTNode>,
//     },
//     While {
//         condition: Condition,
//         block: Vec<ASTNode>,
//     },
// }
//
// pub enum Condition {
//     Equals(Expression, Expression),
//     LessThan(Expression, Expression),
//     GreaterThan(Expression, Expression),
// }
