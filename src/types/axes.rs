use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Axes {
    pub x: u16,
    pub y: u16,
}

impl Axes {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

impl Display for Axes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[{};{}H", self.y, self.x)
    }
}
