mod constants;
mod helpers;
mod menu;
mod types;

use std::io::{self, Write, stdout};

use crossterm::event::{Event, EventStream, KeyCode};
use futures::StreamExt;
use tokio::sync::mpsc::{Receiver, Sender, channel, error::TryRecvError};
use tokio::time::{Duration, interval};

use crate::helpers::{make_even_by_substracting, reset_terminal, setup_terminal};
use crate::menu::{MenuChoice, menu};
use crate::types::{
    Axes, Direction, Fruit, ImpactError, Palette, Snake, Theme, Window, is_going_to_eat_fruit,
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
        Ok(GameResult::Impact(_)) => {}
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

async fn show_start_screen(
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

async fn run_app(mut rx: Receiver<Event>) -> Result<GameResult, AppError> {
    let style = Theme::OrganicV1;
    let palette = style.palette();

    let (width, height) = crossterm::terminal::size()?;

    let mut window = Window::new(width, height, palette.window);

    if let MenuChoice::Quit = show_start_screen(&mut rx, &window).await? {
        return Ok(GameResult::Quit);
    };

    run(&mut rx, &mut window, palette).await
}

async fn run(
    rx: &mut Receiver<Event>,
    window: &mut Window,
    palette: Palette,
) -> Result<GameResult, AppError> {
    let mut snake = Snake::new(
        Axes::new(
            make_even_by_substracting(window.width / 4),
            window.height / 2,
        ),
        Direction::Right,
        constants::INITIAL_SNAKE_LENGTH,
        constants::INITIAL_SNAKE_LIVES,
        palette.snake,
    );
    let mut grow: bool;

    let mut fruit = Fruit::random_generate(window, palette.food);

    let speed = 60;
    let frame_duration = Duration::from_millis(speed);

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

        grow = if is_going_to_eat_fruit(&snake, &fruit) && !invincible {
            fruit.regenerate(window);
            true
        } else {
            false
        };

        if let Err(impact) = snake.update(window, grow) {
            break Ok(GameResult::Impact(impact));
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
    Impact(ImpactError),
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
