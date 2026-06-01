use std::io::{self, Write, stdout};

use crossterm::{ExecutableCommand, QueueableCommand};

use crate::{
    constants::SQUARE_GLYPH,
    types::{Axes, palette::PaletteColors, render::Render},
};

#[derive(Debug)]
pub struct Window {
    pub width: u16,
    pub height: u16,
    pub bg_start: Axes,
    pub bg_end: Axes,
}

impl Window {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            bg_start: Axes::new(2, 1),
            bg_end: Axes::new(
                width - { if width.is_multiple_of(2) { 2 } else { 3 } },
                height - 1,
            ),
        }
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        self.bg_end.x = width - 2;
        self.bg_end.y = height - 1;
    }

    pub fn render<T: Render>(&self, renderable: &T) -> Result<(), io::Error> {
        renderable.render(self)
    }

    pub fn clear_screen(&self) -> Result<(), io::Error> {
        stdout().execute(crossterm::terminal::Clear(
            crossterm::terminal::ClearType::All,
        ))?;
        Ok(())
    }

    pub fn set_background_color(&self, palette: &PaletteColors) -> Result<(), io::Error> {
        stdout().execute(crossterm::style::SetBackgroundColor(palette.background))?;
        self.clear_screen()
    }

    pub fn draw_borders(&self, palette: &PaletteColors) -> Result<(), io::Error> {
        let mut stdout = stdout();

        stdout.queue(crossterm::style::SetBackgroundColor(palette.borders))?;

        let top_down = String::from(SQUARE_GLYPH).repeat((self.width).into());
        for i in [0, self.height - 1] {
            stdout
                .queue(crossterm::cursor::MoveTo(0, i))?
                .queue(crossterm::style::Print(&top_down))?;
        }

        let square = "  ";
        let cond = !self.width.is_multiple_of(2);

        for i in 1..(self.height - 1) {
            stdout
                .queue(crossterm::cursor::MoveTo(0, i))?
                .queue(crossterm::style::Print(&square))?
                .queue(crossterm::cursor::MoveTo(self.bg_end.x, i))?
                .queue(crossterm::style::Print(&square))?;

            if cond {
                stdout.queue(crossterm::style::Print(&SQUARE_GLYPH))?;
            }
        }

        stdout.flush()
    }
}
