use std::io::{self, Write, stdout};

use crossterm::{ExecutableCommand, QueueableCommand};

use crate::{
    constants::SQUARE_GLYPH,
    types::{Axes, palette::WindowPalette, render::Render},
};

#[derive(Debug)]
pub struct Window {
    pub width: u16,
    pub height: u16,
    pub bg_start: Axes,
    pub bg_end: Axes,
    pub palette: WindowPalette,
}

impl Window {
    pub fn new(width: u16, height: u16, palette: WindowPalette) -> Self {
        Self {
            width,
            height,
            bg_start: Axes::new(2, 1),
            bg_end: Axes::new(
                width - { if width.is_multiple_of(2) { 2 } else { 3 } },
                height - 1,
            ),
            palette,
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

    pub fn set_background_color(&self) -> Result<(), io::Error> {
        stdout().execute(crossterm::style::SetBackgroundColor(
            self.palette.background,
        ))?;
        self.clear_screen()
    }

    pub fn draw_borders(&self, score: usize) -> Result<(), io::Error> {
        let mut stdout = stdout();

        stdout.queue(crossterm::style::SetBackgroundColor(self.palette.borders))?;

        let horizontal_down = String::from(SQUARE_GLYPH).repeat(self.width.into());
        stdout
            .queue(crossterm::cursor::MoveTo(0, self.height - 1))?
            .queue(crossterm::style::Print(&horizontal_down))?;

        let score_text = format!(" Score: {:^5} ", score);
        let score_len = score_text.len() as u16;
        let side_len = (self.width - score_len) / 2;
        let side = String::from(SQUARE_GLYPH).repeat(side_len.into());

        stdout
            .queue(crossterm::cursor::MoveTo(0, 0))?
            .queue(crossterm::style::Print(&side))?
            .queue(crossterm::style::SetBackgroundColor(
                self.palette.background,
            ))?
            .queue(crossterm::style::SetForegroundColor(
                self.palette.score_text,
            ))?
            .queue(crossterm::style::SetAttribute(
                crossterm::style::Attribute::Bold,
            ))?
            .queue(crossterm::style::Print(&score_text))?
            .queue(crossterm::style::SetAttribute(
                crossterm::style::Attribute::Reset,
            ))?
            .queue(crossterm::style::SetForegroundColor(
                crossterm::style::Color::Reset,
            ))?
            .queue(crossterm::style::SetBackgroundColor(self.palette.borders))?
            .queue(crossterm::style::Print(&side))?;

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
