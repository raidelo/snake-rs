use std::io::{self, Write, stdout};

use crossterm::QueueableCommand;

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
