use std::io::{self, Write, stdout};

#[derive(Debug)]
pub struct Cursor {
    hidden: bool,
}

impl Cursor {
    pub fn new() -> Self {
        Cursor { hidden: true }
    }

    pub fn hide(&mut self) -> Result<(), io::Error> {
        stdout().write_all("\x1b[?25l".as_bytes())?;
        stdout().flush()?;
        self.hidden = false;

        Ok(())
    }

    pub fn show(&mut self) -> Result<(), io::Error> {
        stdout().write_all("\x1b[?25h".as_bytes())?;
        stdout().flush()?;
        self.hidden = false;

        Ok(())
    }
}
