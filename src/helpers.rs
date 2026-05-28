use std::{
    fmt::Display,
    io::{self, Write, stdin, stdout},
};

use crate::types::{Axes, Window};

pub fn random_pos(window: &Window) -> Axes {
    Axes::new(
        rand::random_range(0..window.width),
        rand::random_range(0..window.height),
    )
}

pub fn render(item: &impl Display, position: &Axes) -> Result<(), io::Error> {
    stdout().write_all(format!("\x1b[{};{}H{}", position.y, position.x, item).as_bytes())?;
    stdout().flush()
}

pub fn clear_screen() -> Result<(), io::Error> {
    stdout().write_all("\x1b[2J".as_bytes())?;
    stdout().flush()
}

pub fn get_input() -> String {
    let mut s = String::new();
    let _ = stdin().read_line(&mut s);
    s
}
