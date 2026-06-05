use std::io;

use crate::{
    constants::SQUARE_GLYPH,
    types::{
        axes::Axes, helpers::random_pos_on_background, palette::FoodPalette, render::Render,
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
            square: Square::new(position, palette.food, SQUARE_GLYPH),
        }
    }

    pub fn random_generate(window: &Window, palette: FoodPalette) -> Self {
        Self::new(random_pos_on_background(window), palette)
    }

    pub fn regenerate(&mut self, window: &Window) {
        self.square.position = random_pos_on_background(window);
    }

    pub fn position(&self) -> Axes {
        self.square.position
    }
}

impl Render for Fruit {
    fn render(&self, window: &Window) -> Result<(), io::Error> {
        self.square.render(window)
    }
}
