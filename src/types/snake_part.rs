use std::io;

use crate::{
    constants::SQUARE_GLYPH,
    types::{
        axes::Axes, color::Color, direction::Direction, render::Render, square::Square,
        window::Window,
    },
};

#[derive(Debug)]
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
        match self.direction {
            Direction::Up => {
                if self.square.position.y > 2 {
                    self.square.position.y -= 1;
                }
            }
            Direction::Down => {
                if self.square.position.y < window.height - 1 {
                    self.square.position.y += 1;
                }
            }
            Direction::Left => {
                if self.square.position.x > 4 {
                    self.square.position.x -= 2;
                }
            }
            Direction::Right => {
                if self.square.position.x < window.width - 4 {
                    self.square.position.x += 2;
                }
            }
        }
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn change_direction(&mut self, direction: Direction) -> bool {
        if direction != self.direction.counter_part() {
            self.set_direction(direction);
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
