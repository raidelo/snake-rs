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

    pub fn render(&self, item: &Square) -> Result<(), io::Error> {
        stdout().write_all(
            format!(
                "\x1b[{};{}H{}",
                item.position.y, item.position.x, item.glyph
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

    pub fn update_pos(&mut self) {
        match self.direction {
            Direction::Up => self.position.y -= 1,
            Direction::Down => self.position.y += 1,
            Direction::Left => self.position.x -= 1,
            Direction::Right => self.position.x += 1,
        }
    }
}

#[derive(Debug)]
pub struct Axes {
    pub x: u16,
    pub y: u16,
}

impl Axes {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
