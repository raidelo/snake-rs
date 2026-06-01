mod constants;
mod helpers;
mod types;

use std::io::{self, Write, stdout};
use std::time::Duration;

use crossterm::QueueableCommand;
use crossterm::event::{Event, KeyCode};
use tokio::sync::mpsc::{Receiver, channel, error::TryRecvError};
use tokio::time::Instant;

use crate::helpers::reset_terminal;
use crate::types::{Axes, Direction, Fruit, ImpactError, PaletteStyle, Snake, Window};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = reset_terminal();

        default_hook(panic_info)
    }));

    let (tx, rx) = channel::<Event>(100);

    let handle = tokio::spawn(display_window(rx));

    while let Ok(event_occurred) = tokio::task::spawn_blocking(|| {
        crossterm::event::poll(Duration::from_millis(constants::POLL_TIME))
    })
    .await?
    {
        if handle.is_finished() {
            break;
        }

        if !event_occurred {
            continue;
        }

        if let Ok(event) = tokio::task::spawn_blocking(crossterm::event::read).await? {
            if tx.send(event).await.is_err() {
                break;
            };
        } else {
            continue;
        };
    }

    match handle.await {
        Ok(Ok(())) => Ok(()),
        Ok(Err(e)) => Err(Box::new(e)),
        Err(err) => {
            if let Ok(panic) = err.try_into_panic() {
                let msg = panic
                    .downcast_ref::<&str>()
                    .map(|s| s.to_string())
                    .or_else(|| panic.downcast_ref::<String>().cloned())
                    .unwrap_or_else(|| "pánico desconocido".to_string());
                Err(Box::new(io::Error::other(msg)))
            } else {
                Err(Box::new(io::Error::other(
                    "tarea cancelada inesperadamente",
                )))
            }
        }
    }
    .map_err(|err| err.into())
}

async fn display_window(mut rx: Receiver<Event>) -> Result<(), io::Error> {
    let _guard = RawModeGuard::new()?;

    let (width, height) = crossterm::terminal::size()?;

    let mut window = Window::new(width, height);
    window.hide_cursor()?;

    let palette = PaletteStyle::Organic.palette();

    let mut snake = Snake::new(
        Axes::new(
            {
                let p = width / 4;
                if p.is_multiple_of(2) { p } else { p - 1 }
            },
            height / 2,
        ),
        Direction::Right,
        constants::INITIAL_SNAKE_LENGTH,
        &palette,
    );

    let fruit = Fruit::random_generate(&window, &palette);

    let speed = 60;
    let frame_duration = Duration::from_millis(speed);
    let mut last_frame = Instant::now();

    let mut impact: Option<ImpactError> = None;

    loop {
        let now = Instant::now();
        if (now - last_frame) >= frame_duration {
            window.set_background_color(&palette)?;
            window.draw_borders(&palette)?;

            if let Err(imp) = snake.update(&window) {
                impact = Some(imp);
                break;
            };
            window.render(&snake)?;

            window.render(&fruit)?;

            last_frame += frame_duration;
        }

        match rx.try_recv() {
            Ok(event) => match event {
                Event::Key(key_event) => {
                    match key_event.code {
                        KeyCode::Up | KeyCode::Char('w') => {
                            snake.change_direction(Direction::Up);
                        }
                        KeyCode::Down | KeyCode::Char('s') => {
                            snake.change_direction(Direction::Down);
                        }
                        KeyCode::Left | KeyCode::Char('a') => {
                            snake.change_direction(Direction::Left);
                        }
                        KeyCode::Right | KeyCode::Char('d') => {
                            snake.change_direction(Direction::Right);
                        }
                        KeyCode::Esc | KeyCode::Char('q') => break,
                        _ => continue,
                    };
                }

                Event::Resize(width, height) => window.resize(width, height),

                _ => (),
            },

            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break,
        };
    }

    window.show_cursor()?;

    reset_terminal()?;

    if impact.is_some() {
        stdout()
            .queue(crossterm::style::SetAttribute(
                crossterm::style::Attribute::Bold,
            ))?
            .queue(crossterm::style::Print("You lost\n"))?
            .queue(crossterm::style::SetAttribute(
                crossterm::style::Attribute::Reset,
            ))?
            .flush()?;
    };

    Ok(())
}

struct RawModeGuard;

impl RawModeGuard {
    fn new() -> Result<Self, io::Error> {
        crossterm::terminal::enable_raw_mode()?;
        Ok(RawModeGuard)
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        for _ in 0..2 {
            if let Ok(()) = crossterm::terminal::disable_raw_mode() {
                break;
            };
        }
    }
}
