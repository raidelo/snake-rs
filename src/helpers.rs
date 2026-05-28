use std::io::stdin;

use crate::types::{Axes, Direction, Window};

pub fn random_pos(window: &Window) -> Axes {
    Axes::new(
        rand::random_range(0..window.width),
        rand::random_range(0..window.height),
    )
}

pub fn random_direction() -> Direction {
    match rand::random_range(1..=4) {
        1 => Direction::Up,
        2 => Direction::Down,
        3 => Direction::Left,
        4 => Direction::Right,
        _ => unreachable!(),
    }
}

pub fn get_input() -> String {
    let mut s = String::new();
    let _ = stdin().read_line(&mut s);
    s
}

pub async fn sleep(millis: u64) -> () {
    tokio::time::sleep(tokio::time::Duration::from_millis(millis)).await;
}
