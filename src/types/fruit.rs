use std::io;

use crate::{
    constants::SQUARE_GLYPH,
    types::{axes::Axes, palette::PaletteStyle, render::Render, square::Square, window::Window},
};

pub struct Fruit(Square);

impl Fruit {
    pub fn new(position: Axes, color: &PaletteStyle) -> Self {
        Self(Square::new(position, color.palette().food, SQUARE_GLYPH))
    }
}

impl Render for Fruit {
    fn render(&self, window: &Window) -> Result<(), io::Error> {
        self.0.render(window)
    }
}
