#[derive(Debug)]
pub struct Window {
    pub width: u16,
    pub height: u16,
}

impl Window {
    pub fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }
}

#[derive(Debug)]
pub struct Square {
    pub position: Axes,
    pub glyph: char,
}

impl Square {
    pub fn new(position: Axes, glyph: char) -> Self {
        Self { position, glyph }
    }
}

#[derive(Debug)]
pub struct Axes {
    pub x: u16,
    pub y: u16,
}

impl Axes {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}
