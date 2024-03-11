use std::f32::consts::PI;

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

    pub fn set_pen_color(&mut self, color: usize) -> Result<(), ExtendedUnsvgError> {
        if !(0..=15).contains(&color) {
            return Err(ExtendedUnsvgError {
                msg: "Colour index must be between 0 and 15 inclusive.".to_string(),
            });
        }

        self.pen_color = color;
        Ok(())
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
        let radians = self.convert_degree_to_radians(self.heading);
        let dx = distance * radians.sin();
        let dy = distance * radians.cos();

        if self.pen_down {
            self.draw_simple_line(self.heading, distance);
        } else {
            self.x -= dx;
            self.y -= dy;
        }
    }

    /// Turtle controls for going backwards
    pub fn back(&mut self, distance: f32) {
        self.forward(-distance);
    }

    /// Turtle controls for going left
    pub fn left(&mut self, distance: f32) {
        let temp_heading = &self.heading - 90;
        let radians = self.convert_degree_to_radians(temp_heading);

        let dx = distance * radians.sin();
        let dy = distance * radians.cos();

        if self.pen_down {
            self.draw_simple_line(temp_heading, distance)
        } else {
            self.x -= dx;
            self.y -= dy;
        }
    }

    /// Turtle controls for going right
    pub fn right(&mut self, distance: f32) {
        let temp_heading = &self.heading + 90;
        let radians = self.convert_degree_to_radians(temp_heading);

        let dx = distance * radians.sin();
        let dy = distance * radians.cos();

        if self.pen_down {
            self.draw_simple_line(temp_heading, distance)
        } else {
            self.x -= dx;
            self.y -= dy;
        }
    }

    /// Converts degrees to radians. Used in calculations for distance when
    /// heading has been changed.
    fn convert_degree_to_radians(&mut self, heading: i32) -> f32 {
        (heading as f32) * (PI / 180.0)
    }

    /// Encapsulate unsvg::Image::draw_simple_line to reduce duplicated code and
    /// make the code more readable.
    fn draw_simple_line(&mut self, heading: i32, distance: f32) {
        let color = COLORS[self.pen_color];
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
    }
}
