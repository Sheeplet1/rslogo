//! Representation of the Logo script as an Abstract Syntax Tree (AST).

#[derive(Debug, Clone)]
pub enum ASTNode {
    Command(Command),
    ControlFlow(ControlFlow),
}

#[derive(Debug, Clone)]
pub enum Expression {
    Float(f32),
    Number(i32),
    Usize(usize),
    Query(Query),
    Variable(String),
    Bool(bool),
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
pub enum Query {
    XCor,
    YCor,
    Heading,
    Color,
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
pub enum Condition {
    Equals(Expression, Expression),
    LessThan(Expression, Expression),
    GreaterThan(Expression, Expression),
}
