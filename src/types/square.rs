use std::io::{self, Write, stdout};

use crossterm::QueueableCommand;

use crate::types::{axes::Axes, color::Color, render::Render, window::Window};

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
        stdout()
            .queue(crossterm::style::SetBackgroundColor(
                crossterm::style::Color::Rgb {
                    r: self.color.r,
                    g: self.color.g,
                    b: self.color.b,
                },
            ))?
            .queue(crossterm::cursor::MoveTo(self.position.x, self.position.y))?
            .queue(crossterm::style::Print(format!(
                "{}{}",
                self.glyph, self.glyph
            )))?;

        stdout().flush()
    }
}
