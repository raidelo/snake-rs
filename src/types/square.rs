use std::io::{self, stdout};

use crate::{
    helpers::write_at_position,
    types::{axes::Axes, color::Color, render::Render, window::Window},
};

#[derive(Debug)]
pub struct Square {
    pub position: Axes,
    pub color: Color,
    pub glyph: char,
}

impl Square {
    pub fn new(position: Axes, color: Color, glyph: char) -> Self {
        Self {
            position,
            color,
            glyph,
        }
    }
}

impl Render for Square {
    fn render(&self, _window: &Window) -> Result<(), io::Error> {
        write_at_position(
            &mut stdout(),
            &self.position,
            &format!("{}{}{}", self.color, self.glyph, self.glyph),
        )
    }
}
