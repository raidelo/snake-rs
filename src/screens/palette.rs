use std::io::{Write, stdout};

use crossterm::{
    QueueableCommand,
    cursor::MoveTo,
    event::{Event, KeyCode},
    style::{Attribute, Color, Print, SetAttribute, SetBackgroundColor, SetForegroundColor},
};
use tokio::sync::mpsc::Receiver;

use crate::{
    AppError,
    types::{Theme, Window},
};

const ALL_THEMES: &[Theme] = &[
    Theme::Classic,
    Theme::NeonV1,
    Theme::NeonV2,
    Theme::OrganicV1,
    Theme::OrganicV2,
    Theme::Dracula,
    Theme::Ocean,
    Theme::Matrix,
    Theme::Sunset,
    Theme::Nord,
    Theme::RetroCga,
];

pub async fn palette_screen(
    rx: &mut Receiver<Event>,
    window: &mut Window,
) -> Result<Option<Theme>, AppError> {
    let mut selected: usize = 0;

    loop {
        window.set_palette(ALL_THEMES[selected].palette().window);

        draw_palette_screen(window, selected)?;

        match rx.recv().await {
            Some(Event::Key(key)) => match key.code {
                KeyCode::Up => {
                    selected = selected.saturating_sub(1);
                }
                KeyCode::Down => {
                    if selected < ALL_THEMES.len() - 1 {
                        selected += 1;
                    }
                }
                KeyCode::Enter => return Ok(Some(ALL_THEMES[selected])),
                KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
                    break Ok(None);
                }
                _ => continue,
            },
            Some(_) => continue,
            None => return Err(AppError::KeyboardListenerDisconnection),
        }
    }
}

fn draw_palette_screen(window: &Window, selected: usize) -> Result<(), AppError> {
    let mut stdout = stdout();

    let cx = window.width / 2;
    let cy = window.height / 2;

    let title = "S E L E C T  T H E M E";
    let tlen = title.chars().count() as u16;

    let max_name_len = ALL_THEMES
        .iter()
        .map(|t| theme_name(t).chars().count())
        .max()
        .unwrap_or(0) as u16;

    let box_width = max_name_len + 6;
    let box_x = cx.saturating_sub(box_width / 2);

    let total_height = ALL_THEMES.len() as u16;
    let box_y = cy.saturating_sub(total_height / 2);

    window.set_background_color()?;

    let palette = &window.palette;
    stdout
        .queue(SetBackgroundColor(palette.background))?
        .queue(SetForegroundColor(palette.score_text))?
        .queue(MoveTo(cx.saturating_sub(tlen / 2), box_y.saturating_sub(2)))?
        .queue(SetAttribute(Attribute::Bold))?
        .queue(Print(title))?
        .queue(SetAttribute(Attribute::NoBold))?;

    for (i, theme) in ALL_THEMES.iter().enumerate() {
        let name = theme_name(theme);
        let name_len = name.chars().count() as u16;
        let x = cx.saturating_sub(name_len / 2);
        let y = box_y + i as u16;

        if i == selected {
            stdout
                .queue(MoveTo(box_x, y))?
                .queue(SetBackgroundColor(palette.score_text))?
                .queue(SetForegroundColor(palette.background))?
                .queue(Print(format!(
                    "  {:^max_name_len$}  ",
                    name,
                    max_name_len = max_name_len as usize
                )))?
                .queue(SetBackgroundColor(palette.background))?
                .queue(SetForegroundColor(palette.score_text))?;
        } else {
            stdout.queue(MoveTo(x, y))?.queue(Print(&name))?;
        }
    }

    stdout
        .queue(SetForegroundColor(Color::Reset))?
        .queue(SetBackgroundColor(Color::Reset))?;

    stdout.flush()?;

    Ok(())
}

fn theme_name(theme: &Theme) -> &'static str {
    match theme {
        Theme::Classic => "Classic",
        Theme::NeonV1 => "Neon V1",
        Theme::NeonV2 => "Neon V2",
        Theme::OrganicV1 => "Organic V1",
        Theme::OrganicV2 => "Organic V2",
        Theme::Dracula => "Dracula",
        Theme::Ocean => "Ocean",
        Theme::Matrix => "Matrix",
        Theme::Sunset => "Sunset",
        Theme::Nord => "Nord",
        Theme::RetroCga => "Retro CGA",
    }
}
