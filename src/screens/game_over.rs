use crossterm::event::{Event, KeyCode};
use tokio::sync::mpsc::Receiver;

use crate::menu::menu;
use crate::types::Window;
use crate::{AppError, constants};

pub async fn game_over_screen(
    rx: &mut Receiver<Event>,
    window: &Window,
) -> Result<GameOverChoice, AppError> {
    menu(
        window,
        "G A M E  O V E R",
        &[
            (constants::KEY_ENTER, constants::LABEL_PLAY_AGAIN),
            (constants::KEY_QUIT, constants::LABEL_QUIT),
        ],
    )?;

    loop {
        match rx.recv().await {
            Some(Event::Key(event)) => match event.code {
                KeyCode::Enter => break Ok(GameOverChoice::Start),
                KeyCode::Char('q') | KeyCode::Char('Q') => break Ok(GameOverChoice::Quit),
                _ => continue,
            },
            Some(_) => continue,
            None => break Err(AppError::KeyboardListenerDisconnection),
        }
    }
}

pub enum GameOverChoice {
    Start,
    Quit,
}
