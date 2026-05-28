mod helpers;
mod types;

use crate::{
    helpers::{get_input, random_direction, random_pos, sleep},
    types::{Square, Window},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let task = tokio::spawn(run());
    task.await??;

    Ok(())
}

async fn run() -> Result<(), std::io::Error> {
    let Some((width, height)) = terminal_size::terminal_size() else {
        return Ok(());
    };

    let mut window = Window::new(width.0, height.0);
    window.hide_cursor()?;

    let pos = random_pos(&window);
    let direction = random_direction();
    let mut square = Square::new(pos, direction);

    loop {
        window.clear_screen()?;
        window.render(&square)?;

        sleep(100).await;

        square.update_pos();
    }
}
