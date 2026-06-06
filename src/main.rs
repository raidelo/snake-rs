mod app;
mod constants;
mod errors;
mod helpers;
mod menu;
mod screens;
mod types;

use std::io;

use crossterm::event::{Event, EventStream};
use futures::StreamExt;
use tokio::sync::mpsc::{Sender, channel};

use crate::app::{AppExit, game_loop};
use crate::errors::AppError;
use crate::helpers::{reset_terminal, setup_terminal};

#[tokio::main]
async fn main() -> AppExit {
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = reset_terminal();
        default_hook(panic_info);
    }));

    match run_app().await {
        Ok(()) => AppExit::Ok,
        Err(AppError::KeyboardListenerDisconnection) => AppExit::Ok,
        Err(AppError::IOError(e)) => AppExit::IOError(e),
    }
}

async fn run_app() -> Result<(), AppError> {
    setup_terminal()?;

    let (tx, rx) = channel::<Event>(100);
    let listener_handler = tokio::task::spawn(keyboard_listener(tx));

    let result = game_loop(rx).await;

    listener_handler.abort();
    reset_terminal()?;

    result
}

async fn keyboard_listener(tx: Sender<Event>) -> Result<(), io::Error> {
    let mut reader = EventStream::new();

    while let Some(event) = reader.next().await {
        if tx.send(event?).await.is_err() {
            break;
        }
    }

    Ok(())
}
