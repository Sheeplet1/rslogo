use crate::error::ParseError;

#[derive(Debug)]
pub enum MovementCommand {
    Forward(f32),
    Back(f32),
    Left(f32),
    Right(f32),
    Turn(i32),
    SetHeading(i32),
    SetX(f32),
    SetY(f32),
}

#[derive(Debug)]
pub enum PenCommand {
    PenUp,
    PenDown,
    SetPenColor(usize),
}

// #[derive(Debug)]
// pub enum Query {
//     Xcor,
//     Ycor,
//     Heading,
//     Color,
// }

#[derive(Debug)]
pub enum Command {
    Movement(MovementCommand),
    Pen(PenCommand),
    Make(String, String),
    AddAssign(String, String),
    // Query,
}

fn parse_command(input: &str) -> Command {
    let mut parts = input.split_whitespace();
    let command = parts.next()?;
    let value = parse_value(parts.next()?).unwrap();

    match command {
        "FORWARD" => Command::Movement(MovementCommand::Forward(value)),
        "BACK" => Command::Movement(MovementCommand::Back(value)),
        "LEFT" => Command::Movement(MovementCommand::Left(value)),
        "RIGHT" => Command::Movement(MovementCommand::Right(value)),
        "TURN" => Command::Movement(MovementCommand::Turn(value as i32)),
        "SETH" => Command::Movement(MovementCommand::SetHeading(value as i32)),
        "SETX" => Command::Movement(MovementCommand::SetX(value)),
        "SETY" => Command::Movement(MovementCommand::SetY(value)),
        "PENUP" => Command::Pen(PenCommand::PenUp),
        "PENDOWN" => Command::Pen(PenCommand::PenDown),
        "SETPENCCOLOR" => Command::Pen(PenCommand::SetPenColor(value as usize)),
    }
}

// Values to parse will result in i32, f32, or usize. BUT the input
// could be a Query. So we need to handle that as well.
fn parse_value(input: &str) -> f32 {
    if input.starts_with(":") {
        // TODO: Query and/or Variable
        todo!()
    }
    let dist: f32 = match input.trim_matches('"').parse() {
        Ok(dist) => dist,
        Err(e) => ParseError {
            msg: format!("Error parsing value: {}", e),
        },
    };

    dist
}
