use crate::types::{Axes, Direction, Fruit, ImpactError, Snake, Window, square::Square};

pub fn round_down_to_even(number: u16) -> u16 {
    if number.is_multiple_of(2) {
        number
    } else {
        number - 1
    }
}

pub fn random_free_position(window: &Window, snake: &Snake) -> Axes {
    let flattened_list: Vec<u16> = collect_free_indices(window, snake);

    let matrix_abstract_pos = flattened_list
        .get(rand::random_range(0..flattened_list.len()))
        .unwrap();

    let y = matrix_abstract_pos / window.width;
    let x = round_down_to_even(matrix_abstract_pos % window.width);

    Axes::new(x, y)
}

fn collect_free_indices(window: &Window, snake: &Snake) -> Vec<u16> {
    let mut list = vec![];

    let mut counter = 0;

    for y in 0..window.height {
        for x in 0..window.width {
            if is_free_position(window, snake, x, y) {
                list.push(counter);
            }
            counter += 1;
        }
    }

    list
}

fn is_free_position(window: &Window, snake: &Snake, x: u16, y: u16) -> bool {
    let pos = Axes::new(x, y);

    window.contains(&pos) && !is_occupied_by_snake(snake, &pos)
}

fn is_occupied_by_snake(snake: &Snake, position: &Axes) -> bool {
    for part in snake.parts.iter() {
        if position == &part.square.position {
            return true;
        }
    }
    false
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
