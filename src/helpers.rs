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

pub fn make_even_by_substracting(number: u16) -> u16 {
    if number.is_multiple_of(2) {
        number
    } else {
        number - 1
    }
}
