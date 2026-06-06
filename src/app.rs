use std::io::{self, Write, stdout};
use std::process::{ExitCode, Termination};

use crossterm::event::{Event, KeyCode};
use tokio::sync::mpsc::{Receiver, error::TryRecvError};
use tokio::time::{Duration, interval};

use crate::constants;
use crate::errors::AppError;
use crate::screens::{
    GameOverChoice, PauseChoice, StartChoice, game_over_screen, palette_screen, pause_screen,
    start_screen,
};
use crate::types::{
    Axes, Direction, Fruit, Palette, Snake, Theme, Window, round_down_to_even, will_eat_fruit,
};

const DEFAULT_THEME: Theme = Theme::OrganicV1;

pub async fn game_loop(mut rx: Receiver<Event>) -> Result<(), AppError> {
    let mut palette = DEFAULT_THEME.palette();

    let (width, height) = crossterm::terminal::size()?;
    let mut window = Window::new(width, height, palette.window);

    loop {
        window.set_background_color()?;
        match start_screen(&mut rx, &window).await? {
            StartChoice::Start => break,
            StartChoice::Palette => {
                if let Some(theme) = palette_screen(&mut rx, &mut window).await? {
                    palette = theme.palette();
                    window.set_palette(palette.window);
                };
            }
            StartChoice::Quit => return Ok(()),
        }
    }

    loop {
        match run(&mut rx, &mut window, palette).await? {
            GameResult::Impact => match game_over_screen(&mut rx, &window).await? {
                GameOverChoice::PlayAgain => continue,
                GameOverChoice::Quit => return Ok(()),
            },
            GameResult::Quit => return Ok(()),
        }
    }
}

pub async fn run(
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
                Event::Key(key_event) => match key_event.code {
                    KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => {
                        snake.change_direction(Direction::Up);
                    }
                    KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') => {
                        snake.change_direction(Direction::Down);
                    }
                    KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('A') => {
                        snake.change_direction(Direction::Left);
                    }
                    KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('D') => {
                        snake.change_direction(Direction::Right);
                    }
                    KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
                        match pause_screen(rx, window).await? {
                            PauseChoice::Continue => {
                                interval.reset();
                            }
                            PauseChoice::Quit => break Ok(GameResult::Quit),
                        }
                    }
                    _ => (),
                },

                Event::Resize(width, height) => window.resize(width, height),

                _ => (),
            },

            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => {
                break Err(AppError::KeyboardListenerDisconnection);
            }
        }

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
        }

        if snake.is_invincible() {
            snake.blink();
        }

        window.render(&snake)?;
        window.render(&fruit)?;

        stdout.flush()?;
    }
}

pub enum GameResult {
    Impact,
    Quit,
}

pub enum AppExit {
    Ok,
    IOError(io::Error),
}

impl Termination for AppExit {
    fn report(self) -> ExitCode {
        match self {
            AppExit::Ok => ExitCode::SUCCESS,
            AppExit::IOError(e) => {
                eprintln!("Error: {e}");
                ExitCode::FAILURE
            }
        }
    }
}
