use std::{
    fmt::Display,
    io::{self, Write, stdout},
};

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

    pub fn render<T: Render>(&self, renderable: &T) -> Result<(), io::Error> {
        renderable.render(self)
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

pub trait Render {
    fn render(&self, window: &Window) -> Result<(), io::Error>;
}

const SNAKE_GLYPH: char = '\u{2588}';

#[derive(Debug)]
pub struct SnakePart {
    pub direction: Direction,
    pub square: Square,
}

impl SnakePart {
    pub fn new(position: Axes, direction: Direction, color: Color) -> Self {
        Self {
            direction,
            square: Square::new(position, color, SNAKE_GLYPH),
        }
    }

    pub fn update_position(&mut self, window: &Window) {
        match self.direction {
            Direction::Up => {
                if self.square.position.y > 1 {
                    self.square.position.y -= 1;
                }
            }
            Direction::Down => {
                if self.square.position.y < window.height {
                    self.square.position.y += 1;
                }
            }
            Direction::Left => {
                if self.square.position.x > 2 {
                    self.square.position.x -= 2;
                }
            }
            Direction::Right => {
                if self.square.position.x < window.width - 2 {
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

impl Display for Axes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[{};{}H", self.y, self.x)
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
    pub parts: Vec<SnakePart>,
}

impl Snake {
    pub fn new(position: Axes, direction: Direction, initial_parts: u16) -> Self {
        let mut parts = vec![SnakePart::new(
            position.clone(),
            direction.clone(),
            default_snake_color(),
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
                    default_snake_color(),
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

#[derive(Debug)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        /* ESC[38;2;{r};{g};{b}m */
        write!(f, "\x1b[38;2;{};{};{}m", self.r, self.g, self.b)
    }
}

fn default_snake_color() -> Color {
    Color::new(100, 255, 100)
}

#[derive(Debug)]
pub struct Square {
    pub position: Axes,
    pub color: Color,
    pub glyph: char,
}

impl Square {
    pub fn new(position: Axes, color: Color, glyph: char) -> Self {
        Self {
            position,
            color,
            glyph,
        }
    }
}

impl Render for Square {
    fn render(&self, _window: &Window) -> Result<(), io::Error> {
        stdout().write_all(
            format!(
                "{}{}{}{}",
                self.position, self.color, self.glyph, self.glyph,
            )
            .as_bytes(),
        )?;
        stdout().flush()
    }
}
