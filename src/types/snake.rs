use std::{
    io,
    time::{Duration, Instant},
};

use crossterm::style::Color;

use crate::{
    constants::INITIAL_SNAKE_LENGTH,
    constants::INVINCIBLE_TIME,
    types::{
        axes::Axes,
        direction::Direction,
        helpers::{get_next_position, is_going_to_impact},
        palette::SnakePalette,
        render::Render,
        snake_part::SnakePart,
        window::Window,
    },
};

#[derive(Debug)]
pub enum ImpactError {
    BodyImpact,
    BorderImpact,
}

#[derive(Debug)]
pub enum BlinkState {
    Normal,
    Blinking,
}

#[derive(Debug)]
pub struct Snake {
    pub parts: Vec<SnakePart>,
    pub palette: SnakePalette,
    pub lives: u8,
    invincible_until: Option<Instant>,
    blink_state: BlinkState,
}

impl Snake {
    pub fn new(
        position: Axes,
        direction: Direction,
        initial_length: u16,
        initial_lives: u8,
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

        Self {
            parts,
            palette,
            lives: initial_lives,
            invincible_until: None,
            blink_state: BlinkState::Normal,
        }
    }

    pub fn update(&mut self, window: &Window, grow: bool) -> Result<(), ImpactError> {
        if let Err(impact) = is_going_to_impact(self, window)
            && !self.is_invincible()
        {
            if self.lives == 1 {
                return Err(impact);
            }

            self.lives -= 1;
            self.invincible_until =
                Some(Instant::now() + Duration::from_millis(INVINCIBLE_TIME.into()));
        };

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

    pub fn score(&self) -> usize {
        self.parts.len() - INITIAL_SNAKE_LENGTH as usize
    }

    pub fn is_invincible(&mut self) -> bool {
        match self.invincible_until {
            Some(until) if until > Instant::now() => true,
            Some(_) => {
                self.invincible_until = None;
                self.blink_state = BlinkState::Normal;
                self.apply_colors(self.palette.head, self.palette.body);
                false
            }
            None => false,
        }
    }

    pub fn blink(&mut self) {
        let (head, body) = match self.blink_state {
            BlinkState::Normal => {
                self.blink_state = BlinkState::Blinking;
                (self.palette.head, self.palette.body)
            }
            BlinkState::Blinking => {
                self.blink_state = BlinkState::Normal;
                (self.palette.head_inv, self.palette.body_inv)
            }
        };

        self.apply_colors(head, body);
    }

    fn apply_colors(&mut self, head: Color, body: Color) {
        self.head_mut().square.color = head;

        for part in self.parts.iter_mut().skip(1) {
            part.square.color = body;
        }
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
