use crate::types::{Axes, Direction, Fruit, ImpactError, Snake, Window, square::Square};

pub fn round_down_to_even(number: u16) -> u16 {
    if number.is_multiple_of(2) {
        number
    } else {
        number - 1
    }
}

pub fn random_pos_on_background(window: &Window) -> Axes {
    Axes::new(
        round_down_to_even(rand::random_range(window.bg_start.x..window.bg_end.x)),
        rand::random_range(window.bg_start.y..window.bg_end.y),
    )
}

pub fn check_impact(snake: &Snake, window: &Window) -> Option<ImpactError> {
    if check_border_impact(snake, window) {
        return Some(ImpactError::BorderImpact);
    }

    if check_body_impact(snake) {
        return Some(ImpactError::BodyImpact);
    }

    None
}

pub fn check_border_impact(snake: &Snake, window: &Window) -> bool {
    let head = snake.head();
    let pos = &head.square.position;

    match head.direction {
        Direction::Up => pos.y == window.bg_start.y,

        Direction::Down => pos.y == window.bg_end.y - 1,

        Direction::Left => pos.x == window.bg_start.x,

        Direction::Right => pos.x == window.bg_end.x - 2,
    }
}

pub fn check_body_impact(snake: &Snake) -> bool {
    let head = snake.head();
    let head_next_pos: Axes = next_position(&head.square, &head.direction);

    let mut part_next_pos: Axes;
    for part in snake.parts.iter().skip(1) {
        part_next_pos = next_position(&part.square, &part.direction);

        if head_next_pos == part_next_pos {
            return true;
        }
    }

    false
}

pub fn next_position(square: &Square, direction: &Direction) -> Axes {
    let mut pos = square.position;
    match direction {
        Direction::Up => pos.y -= 1,
        Direction::Down => pos.y += 1,
        Direction::Left => pos.x -= 2,
        Direction::Right => pos.x += 2,
    }
    pos
}

pub fn will_eat_fruit(snake: &Snake, fruit: &Fruit) -> bool {
    let head = snake.head();

    next_position(&head.square, &head.direction) == fruit.position()
}
