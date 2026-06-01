use std::io;

use crate::{
    constants::SQUARE_GLYPH,
    helpers::random_pos_on_background,
    types::{axes::Axes, palette::PaletteColors, render::Render, square::Square, window::Window},
};

#[derive(Debug)]
pub struct Fruit(Square);

impl Fruit {
    pub fn new(position: Axes, color: &PaletteColors) -> Self {
        Self(Square::new(position, color.food, SQUARE_GLYPH))
    }

    pub fn random_generate(window: &Window, color: &PaletteColors) -> Self {
        Self::new(random_pos_on_background(window), color)
    }
}

impl Render for Fruit {
    fn render(&self, window: &Window) -> Result<(), io::Error> {
        self.0.render(window)
    }
}
