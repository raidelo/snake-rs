use std::{fmt::Display, io};

use crate::types::{Axes, Window};

pub fn random_pos_on_background(window: &Window) -> Axes {
    Axes::new(
        rand::random_range(window.bg_start.x..window.bg_end.x),
        rand::random_range(window.bg_start.y..window.bg_end.y),
    )
}

pub fn make_width_even(width: u16) -> u16 {
    if width.is_multiple_of(2) {
        width
    } else {
        width - 1
    }
}

pub fn write_at_position(
    writer: &mut impl io::Write,
    position: &Axes,
    content: &impl Display,
) -> Result<(), io::Error> {
    write!(writer, "{}{}", position, content)?;
    writer.flush()
}
