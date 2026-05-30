use std::io;

use crate::types::window::Window;

pub trait Render {
    fn render(&self, window: &Window) -> Result<(), io::Error>;
}
