use std::io;

use crossterm::style::Color;

use crate::{
    constants::SQUARE_GLYPH,
    types::{
        axes::Axes, direction::Direction, helpers::next_position, render::Render, square::Square,
        window::Window,
    },
};

#[derive(Debug, Clone)]
pub struct SnakePart {
    pub direction: Direction,
    pub square: Square,
}

impl SnakePart {
    pub fn new(position: Axes, direction: Direction, color: Color) -> Self {
        Self {
            direction,
            square: Square::new(position, color, SQUARE_GLYPH),
        }
    }

    pub fn update_position(&mut self, window: &Window) {
        let next_pos = next_position(&self.square, &self.direction);
        if window.contains(&next_pos) {
            self.square.position = next_pos;
        }
    }

    pub fn change_direction(&mut self, direction: Direction) -> bool {
        if direction != self.direction.opposite() {
            self.direction = direction;
            true
        } else {
            false
        }
    }
}

impl Render for SnakePart {
    fn render(&self, window: &Window) -> Result<(), io::Error> {
        self.square.render(window)
    }
}
