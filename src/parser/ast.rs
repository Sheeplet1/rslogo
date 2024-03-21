//! Representation of the Logo script as an Abstract Syntax Tree (AST).

#[derive(Debug, Clone)]
pub enum ASTNode {
    Command(Command),
    ControlFlow(ControlFlow),
    ProcedureDefinition {
        name: String,
        args: Vec<String>,
        block: Vec<ASTNode>,
    },
    ProcedureCall {
        name: String,
        args: Vec<Expression>,
    },
}

#[derive(Debug, Clone)]
pub enum Expression {
    Float(f32),
    Number(i32),
    Usize(usize),
    Query(Query),
    Variable(String),
    Math(Box<Math>),
    Arg(String),
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
pub enum Condition {
    Equals(Expression, Expression),
    LessThan(Expression, Expression),
    GreaterThan(Expression, Expression),
    And(Expression, Expression),
    Or(Expression, Expression),
}
