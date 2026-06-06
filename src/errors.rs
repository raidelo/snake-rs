use std::io;

pub enum AppError {
    IOError(io::Error),
    KeyboardListenerDisconnection,
}

impl From<io::Error> for AppError {
    fn from(value: io::Error) -> Self {
        Self::IOError(value)
    }
}
