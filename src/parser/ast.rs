//! Representation of the Logo script as an Abstract Syntax Tree (AST).

#[derive(Debug, Clone)]
pub enum ASTNode {
    Command(Command),
    ControlFlow(ControlFlow),
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
    AddAssign(String, Expression),
}

#[derive(Debug, Clone)]
pub enum Expression {
    Float(f32),
    // NOTE: By design, the Number and Usize are not strictly necessary, but
    // are used to make the parser more readable. This  is a design limitation.
    Number(i32),
    Usize(usize),
    Query(Query),
    Variable(String),
    Math(Box<Math>),
}

#[derive(Debug, Clone)]
pub enum Query {
    XCor,
    YCor,
    Heading,
    Color,
}

#[derive(Debug, Clone)]
/// Mathematical operators.
pub enum Math {
    Add(Expression, Expression),
    Sub(Expression, Expression),
    Mul(Expression, Expression),
    Div(Expression, Expression),
    Eq(Expression, Expression),
    Lt(Expression, Expression),
    Gt(Expression, Expression),
    Ne(Expression, Expression),
    And(Expression, Expression),
    Or(Expression, Expression),
}

#[derive(Debug, Clone)]
pub enum ControlFlow {
    If {
        condition: Condition,
        block: Vec<ASTNode>,
    },
    While {
        condition: Condition,
        block: Vec<ASTNode>,
    },
}

#[derive(Debug, Clone)]
/// Conditions are used to control the flow of execution.
pub enum Condition {
    Equals(Expression, Expression),
    LessThan(Expression, Expression),
    GreaterThan(Expression, Expression),
    And(Expression, Expression),
    Or(Expression, Expression),
}
