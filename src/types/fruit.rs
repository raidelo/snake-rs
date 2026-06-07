use std::io;

use crate::{
    constants::CELL_GLYPH,
    types::{
        Snake, axes::Axes, helpers::random_free_position, palette::FoodPalette, render::Render,
        square::Square, window::Window,
    },
};

#[derive(Debug)]
pub struct Fruit {
    square: Square,
}

impl Fruit {
    pub fn new(position: Axes, palette: FoodPalette) -> Self {
        Self {
            square: Square::new(position, palette.food, CELL_GLYPH),
        }
    }

    pub fn random_generate(window: &Window, snake: &Snake, palette: FoodPalette) -> Self {
        Self::new(random_free_position(window, snake), palette)
    }

    pub fn regenerate(&mut self, window: &Window, snake: &Snake) {
        self.square.position = random_free_position(window, snake);
    }

    pub fn position(&self) -> Axes {
        self.square.position
    }

    pub fn set_palette(&mut self, palette: FoodPalette) {
        self.square.color = palette.food;
    }
}

impl Render for Fruit {
    fn render(&self, window: &Window) -> Result<(), io::Error> {
        self.square.render(window)
    }
}
