use std::io;

use crate::types::{
    axes::Axes, direction::Direction, palette::PaletteColors, render::Render,
    snake_part::SnakePart, window::Window,
};

pub struct Snake {
    pub parts: Vec<SnakePart>,
}

impl Snake {
    pub fn new(
        position: Axes,
        direction: Direction,
        initial_parts: u16,
        palette: &PaletteColors,
    ) -> Self {
        let mut parts = vec![SnakePart::new(
            position.clone(),
            direction.clone(),
            palette.snake_head.clone(),
        )];

        if initial_parts != 0 {
            for i in 1..initial_parts {
                let ndirection = direction.clone();

                parts.push(SnakePart::new(
                    match ndirection {
                        Direction::Up => Axes {
                            y: position.y.saturating_add(i),
                            ..position
                        },
                        Direction::Down => Axes {
                            y: position.y.saturating_sub(i),
                            ..position
                        },
                        Direction::Left => Axes {
                            x: position.x.saturating_add(i * 2),
                            ..position
                        },
                        Direction::Right => Axes {
                            x: position.x.saturating_sub(i * 2),
                            ..position
                        },
                    },
                    ndirection,
                    palette.snake_body.clone(),
                ));
            }
        }

        Self { parts }
    }

    pub fn update(&mut self, window: &Window) {
        for square in self.parts.iter_mut() {
            square.update_position(window);
        }

        let mut left: Option<Direction> = None;
        let mut right: Option<Direction>;

        for square in self.parts.iter_mut() {
            right = Some(square.direction.clone());

            if let Some(ndirection) = left {
                square.direction = ndirection;
            }

            left = right;
        }
    }

    pub fn change_direction(&mut self, direction: Direction) -> bool {
        self.parts
            .get_mut(0)
            .expect("the snake must have at least 1 square of lenght")
            .change_direction(direction)
    }
}

impl Render for Snake {
    fn render(&self, window: &Window) -> Result<(), io::Error> {
        for part in self.parts.iter() {
            window.render(part)?;
        }

        Ok(())
    }
}
