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

pub fn make_even_by_substracting(number: u16) -> u16 {
    if number.is_multiple_of(2) {
        number
    } else {
        number - 1
    }
}
