use std::io::{self, Write, stdout};

#[derive(Debug)]
pub struct Cursor {
    hidden: bool,
}

impl Cursor {
    pub fn new() -> Self {
        Cursor { hidden: true }
    }

    pub fn hide(&mut self) -> Result<(), io::Error> {
        stdout().write_all("\x1b[?25l".as_bytes())?;
        stdout().flush()?;
        self.hidden = false;

        Ok(())
    }

    pub fn show(&mut self) -> Result<(), io::Error> {
        stdout().write_all("\x1b[?25h".as_bytes())?;
        stdout().flush()?;
        self.hidden = false;

        Ok(())
    }
}

#[derive(Debug)]
pub struct Window {
    pub width: u16,
    pub height: u16,
    cursor: Cursor,
}

impl Window {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            cursor: Cursor::new(),
        }
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
    }

    pub fn render_square(&self, item: &Square) -> Result<(), io::Error> {
        stdout().write_all(
            format!(
                "\x1b[{};{}H{}{}",
                item.position.y, item.position.x, item.glyph, item.glyph
            )
            .as_bytes(),
        )?;
        stdout().flush()
    }

    pub fn clear_screen(&self) -> Result<(), io::Error> {
        stdout().write_all("\x1b[2J".as_bytes())?;
        stdout().flush()
    }

    pub fn hide_cursor(&mut self) -> Result<(), io::Error> {
        self.cursor.hide()
    }

    pub fn show_cursor(&mut self) -> Result<(), io::Error> {
        self.cursor.show()
    }
}

const SNAKE_GLYPH: char = '\u{2588}';

#[derive(Debug)]
pub struct Square {
    pub glyph: char,
    pub position: Axes,
    pub direction: Direction,
}

impl Square {
    pub fn new(position: Axes, direction: Direction) -> Self {
        Self {
            glyph: SNAKE_GLYPH,
            position,
            direction,
        }
    }

    pub fn update_position(&mut self, window: &Window) {
        match self.direction {
            Direction::Up => {
                if self.position.y > 0 {
                    self.position.y -= 1;
                }
            }
            Direction::Down => {
                if self.position.y < window.height - 1 {
                    self.position.y += 1;
                }
            }
            Direction::Left => {
                if self.position.x > 1 {
                    self.position.x -= 2;
                }
            }
            Direction::Right => {
                if self.position.x < window.width - 1 {
                    self.position.x += 2;
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

#[derive(Debug, Clone)]
pub struct Axes {
    pub x: u16,
    pub y: u16,
}

impl Axes {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn counter_part(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

pub struct Snake {
    pub parts: Vec<Square>,
}

impl Snake {
    pub fn new(position: Axes, direction: Direction, initial_parts: u16) -> Self {
        let mut parts = vec![Square::new(position.clone(), direction.clone())];

        if initial_parts != 0 {
            for i in 1..initial_parts {
                let ndirection = direction.clone();

                parts.push(Square::new(
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
