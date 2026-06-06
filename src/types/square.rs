use std::io::{self, stdout};

use crossterm::{QueueableCommand, style::Color};

use crate::types::{axes::Axes, render::Render, window::Window};

#[derive(Debug, Clone)]
pub struct Square {
    pub position: Axes,
    pub color: Color,
    pub glyph: String,
}

impl Square {
    pub fn new(position: Axes, color: Color, glyph: char) -> Self {
        Self {
            position,
            color,
            glyph: format!("{}{}", glyph, glyph),
        }
    }
}

impl Render for Square {
    fn render(&self, _window: &Window) -> Result<(), io::Error> {
        stdout()
            .queue(crossterm::style::SetBackgroundColor(self.color))?
            .queue(crossterm::cursor::MoveTo(self.position.x, self.position.y))?
            .queue(crossterm::style::Print(&self.glyph))?;
        Ok(())
    }
}
