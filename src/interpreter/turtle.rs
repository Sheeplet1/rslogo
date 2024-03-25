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
//! let turtle = Turtle::new(&mut image);
//! ```

use unsvg::{Image, COLORS};

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
    pub fn new(image: &mut Image) -> Turtle {
        let (width, height) = image.get_dimensions();
        Turtle {
            x: (width / 2) as f32,
            y: (height / 2) as f32,
            heading: 0,
            pen_down: false,
            pen_color: 7,
            image,
        }
    }

    pub fn pen_down(&mut self) {
        self.pen_down = true;
    }

    pub fn pen_up(&mut self) {
        self.pen_down = false;
    }

    pub fn set_pen_color(&mut self, color: usize) {
        self.pen_color = color;
    }

    /// Degrees are not normalised.
    pub fn turn(&mut self, degrees: i32) {
        self.heading += degrees;
    }

    /// Degrees are not normalised.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_turtle() {
        let width: u32 = 100;
        let height: u32 = 100;
        let mut image = Image::new(width, height);

        let turtle = Turtle::new(&mut image);

        assert_eq!(turtle.x, width as f32 / 2.0);
        assert_eq!(turtle.y, height as f32 / 2.0);
        assert_eq!(turtle.heading, 0);
        assert!(!turtle.pen_down);
        assert_eq!(turtle.pen_color, 7);
    }

    #[test]
    fn test_pen_down() {
        let mut image = Image::new(100, 100);
        let mut turtle = Turtle::new(&mut image);

        assert!(!turtle.pen_down);
        turtle.pen_down();
        assert!(turtle.pen_down);
    }

    #[test]
    fn test_pen_up() {
        let mut image = Image::new(100, 100);
        let mut turtle = Turtle::new(&mut image);

        assert!(!turtle.pen_down);
        turtle.pen_down();
        assert!(turtle.pen_down);
        turtle.pen_up();
        assert!(!turtle.pen_down);
    }

    #[test]
    fn test_set_pen_color() {
        let mut image = Image::new(100, 100);
        let mut turtle = Turtle::new(&mut image);

        assert_eq!(turtle.pen_color, 7);
        turtle.set_pen_color(0);
        assert_eq!(turtle.pen_color, 0);
    }

    #[test]
    fn test_turn() {
        let mut image = Image::new(100, 100);
        let mut turtle = Turtle::new(&mut image);

        assert_eq!(turtle.heading, 0);
        turtle.turn(90);
        assert_eq!(turtle.heading, 90);
    }

    #[test]
    fn test_set_heading() {
        let mut image = Image::new(100, 100);
        let mut turtle = Turtle::new(&mut image);

        assert_eq!(turtle.heading, 0);
        turtle.set_heading(90);
        assert_eq!(turtle.heading, 90);
    }

    #[test]
    fn test_set_x() {
        let mut image = Image::new(100, 100);
        let mut turtle = Turtle::new(&mut image);

        assert_eq!(turtle.x, 50.0);
        turtle.set_x(10.0);
        assert_eq!(turtle.x, 10.0);
    }

    #[test]
    fn test_set_y() {
        let mut image = Image::new(100, 100);
        let mut turtle = Turtle::new(&mut image);

        assert_eq!(turtle.y, 50.0);
        turtle.set_y(10.0);
        assert_eq!(turtle.y, 10.0);
    }

    #[test]
    fn test_forward() {
        let mut image = Image::new(100, 100);
        let mut turtle = Turtle::new(&mut image);

        assert_eq!(turtle.x, 50.0);
        assert_eq!(turtle.y, 50.0);
        turtle.forward(10.0);
        assert_eq!(turtle.x, 50.0);
        assert_eq!(turtle.y, 40.0);
    }

    #[test]
    fn test_back() {
        let mut image = Image::new(100, 100);
        let mut turtle = Turtle::new(&mut image);

        assert_eq!(turtle.x, 50.0);
        assert_eq!(turtle.y, 50.0);
        turtle.back(10.0);
        assert_eq!(turtle.x, 50.0);
        assert_eq!(turtle.y, 60.0);
    }

    #[test]
    fn test_left() {
        let mut image = Image::new(100, 100);
        let mut turtle = Turtle::new(&mut image);

        assert_eq!(turtle.x, 50.0);
        assert_eq!(turtle.y, 50.0);
        turtle.left(10.0);
        assert_eq!(turtle.x, 40.0);
        assert_eq!(turtle.y, 50.0);
    }

    #[test]
    fn test_right() {
        let mut image = Image::new(100, 100);
        let mut turtle = Turtle::new(&mut image);

        assert_eq!(turtle.x, 50.0);
        assert_eq!(turtle.y, 50.0);
        turtle.right(10.0);
        assert_eq!(turtle.x, 60.0);
        assert_eq!(turtle.y, 50.0);
    }

    #[test]
    fn test_move_turtle() {
        let mut image = Image::new(100, 100);
        let mut turtle = Turtle::new(&mut image);

        assert_eq!(turtle.x, 50.0);
        assert_eq!(turtle.y, 50.0);
        turtle.move_turtle(0, 10.0);
        assert_eq!(turtle.x, 50.0);
        assert_eq!(turtle.y, 40.0);
    }
}
