use crossterm::event::{Event, KeyCode};
use tokio::sync::mpsc::Receiver;

use crate::menu::menu;
use crate::types::Window;
use crate::{AppError, constants};

pub async fn start_screen(
    rx: &mut Receiver<Event>,
    window: &Window,
) -> Result<MenuChoice, AppError> {
    window.set_background_color()?;

    menu(
        window,
        "S N A K E",
        &[
            (constants::KEY_ENTER, constants::LABEL_PLAY),
            (constants::KEY_QUIT, constants::LABEL_QUIT),
        ],
    )?;

    loop {
        match rx.recv().await {
            Some(Event::Key(event)) => match event.code {
                KeyCode::Enter => break Ok(MenuChoice::Start),
                KeyCode::Char('q') | KeyCode::Char('Q') => break Ok(MenuChoice::Quit),
                _ => continue,
            },
            Some(_) => continue,
            None => break Err(AppError::KeyboardListenerDisconnection),
        }
    }
}

pub enum MenuChoice {
    Start,
    Quit,
}
