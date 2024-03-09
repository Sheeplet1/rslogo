pub enum ASTNode {
    Command(Command),
    // Operation(Operation), // TODO: Fill out for later part
    Value(Value),
}

pub enum Command {
    Forward(Box<ASTNode>),
    Back(Box<ASTNode>),
    Left(Box<ASTNode>),
    Right(Box<ASTNode>),
    PenUp(Box<ASTNode>),
    PenDown(Box<ASTNode>),
    SetPenColor(Box<ASTNode>),
    Turn(Box<ASTNode>),
    SetHeading(Box<ASTNode>),
    SetX(Box<ASTNode>),
    SetY(Box<ASTNode>),
}

// pub enum Operation {
// }

pub enum Value {
    Number(f32),
    Color(usize),
}
