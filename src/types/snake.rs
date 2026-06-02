use std::io;

use crate::types::{
    axes::Axes,
    direction::Direction,
    helpers::{get_next_position, is_going_to_impact},
    palette::SnakePalette,
    render::Render,
    snake_part::SnakePart,
    window::Window,
};

#[derive(Debug)]
pub enum ImpactError {
    BodyImpact,
    BorderImpact,
}

#[derive(Debug)]
pub struct Snake {
    pub parts: Vec<SnakePart>,
    pub palette: SnakePalette,
}

impl Snake {
    pub fn new(
        position: Axes,
        direction: Direction,
        initial_length: u16,
        palette: SnakePalette,
    ) -> Self {
        let mut parts = vec![SnakePart::new(position, direction, palette.head)];

        for i in 1..initial_length {
            parts.push(SnakePart::new(
                match direction {
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
                direction,
                palette.body,
            ));
        }

        Self { parts, palette }
    }

    pub fn update(&mut self, window: &Window, grow: bool) -> Result<(), ImpactError> {
        is_going_to_impact(self, window)?;

        if grow {
            let color = self.palette.body;

            let head = self.head_mut();
            let mut new_head = head.clone();

            head.square.color = color;

            new_head.square.position = get_next_position(&new_head.square, &new_head.direction);

            self.parts.insert(0, new_head);

            return Ok(());
        }

        for square in self.parts.iter_mut() {
            square.update_position(window);
        }

        let mut left: Option<Direction> = None;
        let mut right: Option<Direction>;

        for square in self.parts.iter_mut() {
            right = Some(square.direction);

            if let Some(direction) = left {
                square.direction = direction;
            }

            left = right;
        }

        Ok(())
    }

    pub fn change_direction(&mut self, direction: Direction) -> bool {
        self.head_mut().change_direction(direction)
    }

    pub fn head(&self) -> &SnakePart {
        self.parts
            .first()
            .expect("the snake must have at least 1 square of lenght")
    }

    pub fn head_mut(&mut self) -> &mut SnakePart {
        self.parts
            .first_mut()
            .expect("the snake must have at least 1 square of lenght")
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
