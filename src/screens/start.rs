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
        &[
            (
                constants::MENU_ENTER_CHOICE,
                constants::MENU_ENTER_CHOICE_VALUE,
            ),
            (
                constants::MENU_QUIT_CHOICE,
                constants::MENU_QUIT_CHOICE_VALUE,
            ),
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
