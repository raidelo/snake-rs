use std::io;

use crossterm::style::Color;

use crate::{
    constants::SQUARE_GLYPH,
    types::{
        axes::Axes, helpers::random_pos_on_background, render::Render, square::Square,
        window::Window,
    },
};

#[derive(Debug)]
pub struct Fruit(Square);

impl Fruit {
    pub fn new(position: Axes, color: Color) -> Self {
        Self(Square::new(position, color, SQUARE_GLYPH))
    }

    pub fn random_generate(window: &Window, color: Color) -> Self {
        Self::new(random_pos_on_background(window), color)
    }

    pub fn regenerate(&mut self, window: &Window) {
        self.0.position = random_pos_on_background(window);
    }

    pub fn position(&self) -> Axes {
        self.0.position
    }
}

impl Render for Fruit {
    fn render(&self, window: &Window) -> Result<(), io::Error> {
        self.0.render(window)
    }
}
