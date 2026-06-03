mod constants;
mod helpers;
mod types;

use std::io::{self, Write, stdout};

use crossterm::QueueableCommand;
use crossterm::event::{Event, EventStream, KeyCode};
use futures::StreamExt;
use tokio::sync::mpsc::{Receiver, Sender, channel, error::TryRecvError};
use tokio::time::{Duration, interval};

use crate::helpers::{make_even_by_substracting, reset_terminal, setup_terminal};
use crate::types::{
    Axes, Direction, Fruit, ImpactError, PaletteStyle, Snake, Window, is_going_to_eat_fruit,
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

    let result = run_game(rx).await;
    listener_handler.abort();

    reset_terminal()?;

    match result {
        Ok(()) => Ok(()),

        Err(GameError::ImpactError(_)) => {
            stdout()
                .queue(crossterm::style::SetAttribute(
                    crossterm::style::Attribute::Bold,
                ))?
                .queue(crossterm::style::Print("You lost\n"))?
                .queue(crossterm::style::SetAttribute(
                    crossterm::style::Attribute::Reset,
                ))?
                .flush()?;
            Ok(())
        }

        Err(GameError::IOError(err)) => Err(Box::new(err) as Box<dyn std::error::Error>),
    }
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

async fn run_game(mut rx: Receiver<Event>) -> Result<(), GameError> {
    let style = PaletteStyle::OrganicV1;
    let (window_palette, snake_palette, food_color) = style.palette().unpack();

    let (width, height) = crossterm::terminal::size()?;

    let mut window = Window::new(width, height, window_palette);

    let mut snake = Snake::new(
        Axes::new(make_even_by_substracting(width / 4), height / 2),
        Direction::Right,
        constants::INITIAL_SNAKE_LENGTH,
        constants::INITIAL_SNAKE_LIVES,
        snake_palette,
    );
    let mut grow: bool;

    let mut fruit = Fruit::random_generate(&window, food_color);

    let speed = 60;
    let frame_duration = Duration::from_millis(speed);

    let mut interval = interval(frame_duration);
    interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

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
                        KeyCode::Esc | KeyCode::Char('q') => break Ok(()),

                        _ => (),
                    };
                }

                Event::Resize(width, height) => window.resize(width, height),

                _ => (),
            },

            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break Ok(()),
        };

        window.set_background_color()?;
        window.draw_borders(snake.score(), snake.lives)?;

        let invincible = snake.is_invincible();

        grow = if is_going_to_eat_fruit(&snake, &fruit) && !invincible {
            fruit.regenerate(&window);
            true
        } else {
            false
        };

        if let Err(imp) = snake.update(&window, grow) {
            break Err(imp.into());
        };

        if snake.is_invincible() {
            snake.blink();
        }

        window.render(&snake)?;

        window.render(&fruit)?;
    }
}

#[allow(unused)]
enum GameError {
    IOError(io::Error),
    ImpactError(ImpactError),
}

impl From<io::Error> for GameError {
    fn from(value: io::Error) -> Self {
        Self::IOError(value)
    }
}

impl From<ImpactError> for GameError {
    fn from(value: ImpactError) -> Self {
        Self::ImpactError(value)
    }
}
