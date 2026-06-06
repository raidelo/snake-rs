use std::io::{self, Write, stdout};

use crossterm::QueueableCommand;

pub fn setup_terminal() -> Result<(), io::Error> {
    crossterm::terminal::enable_raw_mode()?;
    stdout()
        .queue(crossterm::cursor::Hide)?
        .queue(crossterm::terminal::EnterAlternateScreen)?
        .flush()
}

pub fn reset_terminal() -> Result<(), io::Error> {
    crossterm::terminal::disable_raw_mode()?;
    stdout()
        .queue(crossterm::cursor::Show)?
        .queue(crossterm::terminal::LeaveAlternateScreen)?
        .flush()
}
