use std::io::{self, Write, stdout};

pub struct Hidden;
pub struct Visible;

pub struct CursorStatus<Status = Visible> {
    _status: std::marker::PhantomData<Status>,
}

impl CursorStatus {
    pub fn new() -> Self {
        CursorStatus::<Visible> {
            _status: std::marker::PhantomData,
        }
    }
}

impl CursorStatus<Visible> {
    pub fn hide(self) -> Result<CursorStatus<Hidden>, io::Error> {
        stdout().write_all("\x1b[?25l".as_bytes())?;
        stdout().flush()?;

        Ok(CursorStatus::<Hidden> {
            _status: std::marker::PhantomData,
        })
    }
}

impl CursorStatus<Hidden> {
    pub fn show(self) -> Result<CursorStatus<Visible>, io::Error> {
        stdout().write_all("\x1b[?25h".as_bytes())?;
        stdout().flush()?;

        Ok(CursorStatus::<Visible> {
            _status: std::marker::PhantomData,
        })
    }
}
