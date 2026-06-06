mod constants;
mod helpers;
mod menu;
mod screens;
mod types;

use std::io::{self, Write, stdout};

use crossterm::event::{Event, EventStream, KeyCode};
use futures::StreamExt;
use tokio::sync::mpsc::{Receiver, Sender, channel, error::TryRecvError};
use tokio::time::{Duration, interval};

use crate::helpers::{reset_terminal, setup_terminal};
use crate::screens::{GameOverChoice, MenuChoice, game_over_screen, start_screen};
use crate::types::{
    Axes, Direction, Fruit, Palette, Snake, Theme, Window, round_down_to_even, will_eat_fruit,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = reset_terminal();

        default_hook(panic_info)
    }));

    let (tx, rx) = channel::<Event>(100);

    setup_terminal()?;

    let listener_handler = tokio::task::spawn(keyboard_listener(tx));

    let result = run_app(rx).await;
    listener_handler.abort();

    reset_terminal()?;

    match result {
        Ok(GameResult::Impact) => {}
        Ok(GameResult::Quit) => {}
        Err(AppError::KeyboardListenerDisconnection) => {}
        Err(AppError::IOError(e)) => return Err(Box::new(e) as Box<dyn std::error::Error>),
    }

    Ok(())
}

async fn keyboard_listener(tx: Sender<Event>) -> Result<(), io::Error> {
    let mut reader = EventStream::new();

    while let Some(event) = reader.next().await {
        if tx.send(event?).await.is_err() {
            break;
        };
    }
    Ok(())
}

async fn run_app(mut rx: Receiver<Event>) -> Result<GameResult, AppError> {
    let style = Theme::OrganicV1;
    let palette = style.palette();

    let (width, height) = crossterm::terminal::size()?;

    let mut window = Window::new(width, height, palette.window);

    if let MenuChoice::Quit = start_screen(&mut rx, &window).await? {
        return Ok(GameResult::Quit);
    };

    loop {
        match run(&mut rx, &mut window, palette).await? {
            GameResult::Impact => match game_over_screen(&mut rx, &window).await? {
                GameOverChoice::Start => continue,

                GameOverChoice::Quit => return Ok(GameResult::Quit),
            },

            GameResult::Quit => return Ok(GameResult::Quit),
        }
    }
}

async fn run(
    rx: &mut Receiver<Event>,
    window: &mut Window,
    palette: Palette,
) -> Result<GameResult, AppError> {
    let mut snake = Snake::new(
        Axes::new(round_down_to_even(window.width / 4), window.height / 2),
        Direction::Right,
        constants::INITIAL_SNAKE_LENGTH,
        constants::INITIAL_SNAKE_LIVES,
        palette.snake,
    );
    let mut grow: bool;

    let mut fruit = Fruit::random_generate(window, palette.food);

    let frame_duration = Duration::from_millis(constants::FRAME_DURATION);

    let mut interval = interval(frame_duration);
    interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

    let mut stdout = stdout();

    loop {
        interval.tick().await;

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
                        KeyCode::Esc | KeyCode::Char('q') => break Ok(GameResult::Quit),

                        _ => (),
                    };
                }

                Event::Resize(width, height) => window.resize(width, height),

                _ => (),
            },

            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => {
                break Err(AppError::KeyboardListenerDisconnection);
            }
        };

        window.set_background_color()?;
        window.draw_borders(snake.score(), snake.lives)?;

        let invincible = snake.is_invincible();

        grow = if will_eat_fruit(&snake, &fruit) && !invincible {
            fruit.regenerate(window);
            true
        } else {
            false
        };

        if let Err(_impact) = snake.update(window, grow) {
            break Ok(GameResult::Impact);
        };

        if snake.is_invincible() {
            snake.blink();
        }

        window.render(&snake)?;

        window.render(&fruit)?;

        stdout.flush()?;
    }
}

enum GameResult {
    Impact,
    Quit,
}

enum AppError {
    IOError(io::Error),
    KeyboardListenerDisconnection,
}

impl From<io::Error> for AppError {
    fn from(value: io::Error) -> Self {
        Self::IOError(value)
    }
}
