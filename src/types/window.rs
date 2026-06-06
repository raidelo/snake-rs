use std::io::{self, stdout};

use crossterm::QueueableCommand;

use crate::{
    constants::{CELL_GLYPH, HEART_GLYPH, MAX_DISPLAYED_LIVES},
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

        self.bg_end = Axes::new(
            width - { if width.is_multiple_of(2) { 2 } else { 3 } },
            height - 1,
        );
    }

    pub fn render<T: Render>(&self, renderable: &T) -> Result<(), io::Error> {
        renderable.render(self)
    }

    pub fn clear_screen(&self) -> Result<(), io::Error> {
        stdout().queue(crossterm::terminal::Clear(
            crossterm::terminal::ClearType::All,
        ))?;
        Ok(())
    }

    pub fn set_background_color(&self) -> Result<(), io::Error> {
        stdout().queue(crossterm::style::SetBackgroundColor(
            self.palette.background,
        ))?;
        self.clear_screen()
    }

    pub fn draw_borders(&self, score: usize, lives: u8) -> Result<(), io::Error> {
        let mut stdout = stdout();

        stdout.queue(crossterm::style::SetBackgroundColor(self.palette.borders))?;

        let horizontal_down = String::from(CELL_GLYPH).repeat(self.width.into());
        stdout
            .queue(crossterm::cursor::MoveTo(0, self.height - 1))?
            .queue(crossterm::style::Print(&horizontal_down))?;

        let hearts = format!("{HEART_GLYPH} ").repeat(if lives <= MAX_DISPLAYED_LIVES {
            lives
        } else {
            MAX_DISPLAYED_LIVES
        } as usize);

        let score_text = format!(" Score: {:^5} | Lives: {}x {} ", score, hearts, lives);
        let score_len = score_text.chars().count() as u16;
        let side_len = (self.width - score_len) / 2;
        let side = String::from(CELL_GLYPH).repeat(side_len.into());

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

        if !(self.width - score_len).is_multiple_of(2) {
            stdout.queue(crossterm::style::Print(&CELL_GLYPH))?;
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
                stdout.queue(crossterm::style::Print(&CELL_GLYPH))?;
            }
        }

        Ok(())
    }

    pub fn contains(&self, value: &Axes) -> bool {
        value.x >= self.bg_start.x
            && value.x <= self.bg_end.x - 2
            && value.y >= self.bg_start.y
            && value.y < self.bg_end.y
    }
}
