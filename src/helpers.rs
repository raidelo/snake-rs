use crate::types::{Axes, Window};

pub fn random_pos_on_background(window: &Window) -> Axes {
    Axes::new(
        rand::random_range(window.bg_start.x..window.bg_end.x),
        rand::random_range(window.bg_start.y..window.bg_end.y),
    )
}
