use crossterm::event::{Event, KeyCode};
use tokio::sync::mpsc::Receiver;

use crate::menu::menu;
use crate::types::Window;
use crate::{AppError, constants};

pub async fn pause_screen(
    rx: &mut Receiver<Event>,
    window: &Window,
) -> Result<PauseChoice, AppError> {
    menu(
        window,
        "P A U S E",
        &[
            (constants::KEY_ENTER, constants::LABEL_CONTINUE),
            (constants::KEY_QUIT, constants::LABEL_QUIT),
        ],
    )?;

    loop {
        match rx.recv().await {
            Some(Event::Key(event)) => match event.code {
                KeyCode::Enter => break Ok(PauseChoice::Continue),
                KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
                    break Ok(PauseChoice::Quit);
                }
                _ => continue,
            },
            Some(_) => continue,
            None => break Err(AppError::KeyboardListenerDisconnection),
        }
    }
}

pub enum PauseChoice {
    Continue,
    Quit,
}
