use std::io::{self, stdout};

use crossterm::ExecutableCommand;

use crate::{
    constants::SQUARE_GLYPH,
    helpers::write_at_position,
    types::{Axes, palette::PaletteColors, render::Render},
};

#[derive(Debug)]
pub struct Window {
    pub width: u16,
    pub height: u16,
}

impl Window {
    pub fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
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

    pub fn hide_cursor(&self) -> Result<(), io::Error> {
        stdout().execute(crossterm::cursor::Hide)?;
        Ok(())
    }

    pub fn show_cursor(&self) -> Result<(), io::Error> {
        stdout().execute(crossterm::cursor::Show)?;
        Ok(())
    }

    pub fn draw_background(&self, palette: &PaletteColors) -> Result<(), io::Error> {
        let mut stdout = stdout();

        let color = &palette.borders;
        let background_color = &palette.background;

        let top_down = format!(
            "{}{}",
            color,
            String::from(SQUARE_GLYPH).repeat(self.width.into())
        );

        let center = format!(
            "{color}{SQUARE_GLYPH}{SQUARE_GLYPH}{background_color}{}{color}{SQUARE_GLYPH}{SQUARE_GLYPH}",
            String::from(SQUARE_GLYPH).repeat((self.width - 4).into())
        );

        write_at_position(&mut stdout, &Axes::new(1, 1), &top_down)?;
        write_at_position(&mut stdout, &Axes::new(1, self.height), &top_down)?;

        for i in 2..(self.height) {
            write_at_position(&mut stdout, &Axes::new(1, i), &center)?;
        }

        Ok(())
    }
}
