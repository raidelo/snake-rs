mod cursor;
mod helpers;
mod types;

use crate::{
    cursor::CursorStatus,
    helpers::{clear_screen, get_input, random_pos, render},
    types::Window,
};

const SNAKE_GLYPH: char = '\u{2588}';

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Some((width, height)) = terminal_size::terminal_size() else {
        return Ok(());
    };

    let window = Window::new(width.0, height.0);

    let cursor = CursorStatus::new().hide()?;

    loop {
        let pos = random_pos(&window);

        let _ = clear_screen();
        let _ = render(&SNAKE_GLYPH, &pos);

        let input = get_input();

        if input.trim() == "q" {
            cursor.show()?;

            break;
        }
    }

    Ok(())
}
