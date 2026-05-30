use std::io::{self, Write, stdout};

use crate::types::{cursor::Cursor, render::Render};

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
