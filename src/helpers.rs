use std::io::{self, Write, stdout};

use crossterm::QueueableCommand;

use crate::types::{Axes, Window};

pub fn random_pos_on_background(window: &Window) -> Axes {
    Axes::new(
        rand::random_range(window.bg_start.x..window.bg_end.x),
        rand::random_range(window.bg_start.y..window.bg_end.y),
    )
}

pub fn reset_terminal() -> Result<(), io::Error> {
    crossterm::terminal::disable_raw_mode()?;
    stdout()
        .queue(crossterm::style::ResetColor)?
        .queue(crossterm::terminal::Clear(
            crossterm::terminal::ClearType::All,
        ))?
        .queue(crossterm::cursor::Show)?
        .queue(crossterm::cursor::MoveTo(0, 0))?
        .flush()
}
