//! This module contains the Turtle struct and its implementation. The Turtle
//! struct is used to represent the turtle in the Logo programming language.
//!
//! The turtle is used to draw lines on the screen, and the turtle's state
//! (position, heading, pen state, etc.) is modified by the Logo commands.
//!
//! # Example
//! The turtle follows the below default state:
//!
//! ```rust
//! use unsvg::Image;
//!
//! let width: f32 = 100.0;
//! let height: f32 = 100.0;
//!
//! let mut image = Image::new(width, height);
//!
//! let mut turtle = Turtle {
//!    x: (width / 2),
//!    y: (height / 2),
//!    heading: 0,
//!    pen_down: false,
//!    pen_color: 7, // White
//!    image: &mut image,
//! }
//! ```

use unsvg::{Image, COLORS};

use crate::errors::ExtendedUnsvgError;

pub struct Turtle<'a> {
    pub x: f32,
    pub y: f32,
    /// Degrees, where 0 is Up/North
    pub heading: i32,
    pub pen_down: bool,
    /// Indexed into a unsvg::COLORS array.
    pub pen_color: usize,
    pub image: &'a mut Image,
}

impl Turtle<'_> {
    pub fn pen_down(&mut self) {
        self.pen_down = true;
    }

    pub fn pen_up(&mut self) {
        self.pen_down = false;
    }

    /// Set the pen color. The color index must be between 0 and 15 inclusive.
    pub fn set_pen_color(&mut self, color: usize) -> Result<(), ExtendedUnsvgError> {
        if !(0..=15).contains(&color) {
            return Err(ExtendedUnsvgError {
                msg: "Colour index must be between 0 and 15 inclusive.".to_string(),
            });
        }

        self.pen_color = color;
        Ok(())
    }

    /// Turn the turtle by the given number of degrees. Degrees are not normalised.
    pub fn turn(&mut self, degrees: i32) {
        self.heading += degrees;
    }

    /// Set the heading of the turtle. Degrees are not normalised.
    pub fn set_heading(&mut self, degrees: i32) {
        self.heading = degrees;
    }

    /// Set the x coordinate of the turtle. Note that even if the pen is down,
    /// the turtle will not draw a line to the new position.
    pub fn set_x(&mut self, x: f32) {
        self.x = x
    }

    /// Set the y coordinate of the turtle. Note that even if the pen is down,
    /// the turtle will not draw a line to the new position.
    pub fn set_y(&mut self, y: f32) {
        self.y = y
    }

    /// Turtle controls for going forwards
    pub fn forward(&mut self, distance: f32) {
        self.move_turtle(self.heading, distance);
    }

    /// Turtle controls for going backwards
    pub fn back(&mut self, distance: f32) {
        self.move_turtle((self.heading + 180) % 360, distance);
    }

    /// Turtle controls for going left
    pub fn left(&mut self, distance: f32) {
        self.move_turtle((self.heading - 90) % 360, distance);
    }

    /// Turtle controls for going right
    pub fn right(&mut self, distance: f32) {
        self.move_turtle((self.heading + 90) % 360, distance);
    }

    /// Move the turtle in the direction of the heading by the given distance.
    /// If the pen is down, draw a line to the new position.
    fn move_turtle(&mut self, heading: i32, distance: f32) {
        let color = COLORS[self.pen_color];
        if self.pen_down {
            match self
                .image
                .draw_simple_line(self.x, self.y, heading, distance, color)
            {
                Ok((x, y)) => {
                    self.x = x;
                    self.y = y;
                }
                Err(e) => panic!("Error drawing line: {:?}", e),
            }
        } else {
            let (end_x, end_y) = unsvg::get_end_coordinates(self.x, self.y, heading, distance);
            self.x = end_x;
            self.y = end_y;
        }
    }
}
