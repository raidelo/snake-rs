use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    #[allow(unused)]
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        /* ESC[38;2;{r};{g};{b}m */
        write!(f, "\x1b[38;2;{};{};{}m", self.r, self.g, self.b)
    }
}
