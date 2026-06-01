use std::io::{self, Write, stdout};

use crossterm::{ExecutableCommand, QueueableCommand};

use crate::types::{Axes, palette::PaletteColors, render::Render};

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
            bg_end: Axes::new(width - 2, height - 1),
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

    pub fn hide_cursor(&self) -> Result<(), io::Error> {
        stdout().execute(crossterm::cursor::Hide)?;
        Ok(())
    }

    pub fn show_cursor(&self) -> Result<(), io::Error> {
        stdout().execute(crossterm::cursor::Show)?;
        Ok(())
    }

    pub fn set_background_color(&self, palette: &PaletteColors) -> Result<(), io::Error> {
        stdout().execute(crossterm::style::SetBackgroundColor(
            crossterm::style::Color::Rgb {
                r: palette.background.r,
                g: palette.background.g,
                b: palette.background.b,
            },
        ))?;
        self.clear_screen()
    }

    pub fn draw_borders(&self, palette: &PaletteColors) -> Result<(), io::Error> {
        let mut stdout = stdout();

        stdout.queue(crossterm::style::SetBackgroundColor(
            crossterm::style::Color::Rgb {
                r: palette.borders.r,
                g: palette.borders.g,
                b: palette.borders.b,
            },
        ))?;

        let top_down = String::from(" ").repeat((self.width).into());
        for i in [0, self.height - 1] {
            stdout
                .queue(crossterm::cursor::MoveTo(0, i))?
                .queue(crossterm::style::Print(&top_down))?;
        }

        let block = "  ";
        for i in 1..(self.height - 1) {
            stdout
                .queue(crossterm::cursor::MoveTo(0, i))?
                .queue(crossterm::style::Print(&block))?
                .queue(crossterm::cursor::MoveTo(self.width - 2, i))?
                .queue(crossterm::style::Print(&block))?;
        }

        stdout.flush()
    }
}
