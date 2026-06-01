use std::io;

use crate::types::{
    axes::Axes, direction::Direction, palette::PaletteColors, render::Render,
    snake_part::SnakePart, window::Window,
};

#[derive(Debug)]
pub enum ImpactError {
    BodyImpact,
    BorderImpact,
}

pub struct Snake {
    pub parts: Vec<SnakePart>,
}

impl Snake {
    pub fn new(
        position: Axes,
        direction: Direction,
        initial_length: u16,
        palette: &PaletteColors,
    ) -> Self {
        let mut parts = vec![SnakePart::new(
            position,
            direction.clone(),
            palette.snake_head,
        )];

        for i in 1..initial_length {
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
                palette.snake_body,
            ));
        }

        Self { parts }
    }

    pub fn update(&mut self, window: &Window) -> Result<(), ImpactError> {
        is_going_to_impact(self, window)?;

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

        Ok(())
    }

    pub fn change_direction(&mut self, direction: Direction) -> bool {
        self.parts
            .first_mut()
            .expect("the snake must have at least 1 square of lenght")
            .change_direction(direction)
    }

    fn head(&self) -> &SnakePart {
        self.parts
            .first()
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

fn is_going_to_impact(snake: &Snake, window: &Window) -> Result<(), ImpactError> {
    if check_border_impact(snake, window) {
        return Err(ImpactError::BorderImpact);
    }

    if check_body_impact(snake) {
        return Err(ImpactError::BodyImpact);
    }

    Ok(())
}

fn check_border_impact(snake: &Snake, window: &Window) -> bool {
    let head = snake.head();
    let pos = &head.square.position;

    match head.direction {
        Direction::Up => pos.y == window.bg_start.y,

        Direction::Down => pos.y == window.bg_end.y - 1,

        Direction::Left => pos.x == window.bg_start.x,

        Direction::Right => pos.x == window.bg_end.x - 2,
    }
}

fn check_body_impact(snake: &Snake) -> bool {
    let head_next_pos: Axes = get_next_position(snake.head());

    let mut part_next_pos: Axes;
    for part in snake.parts.iter().skip(1) {
        part_next_pos = get_next_position(part);

        if head_next_pos == part_next_pos {
            return true;
        }
    }

    false
}

fn get_next_position(snake_part: &SnakePart) -> Axes {
    let mut pos = snake_part.square.position;
    match snake_part.direction {
        Direction::Up => pos.y -= 1,
        Direction::Down => pos.y += 1,
        Direction::Left => pos.x -= 2,
        Direction::Right => pos.x += 2,
    }
    pos
}
