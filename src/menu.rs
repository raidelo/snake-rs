use std::io::{self, Write, stdout};

use crossterm::{
    QueueableCommand,
    cursor::MoveTo,
    style::{Attribute, Color, Print, SetAttribute, SetForegroundColor},
};

use crate::{
    constants::{MENU_ARROW, MENU_SEP},
    types::Window,
};

pub fn menu(window: &Window, title: &str, options: &[(&str, &str)]) -> Result<(), io::Error> {
    let mut stdout = stdout();

    let palette = &window.palette;
    let cx = window.width / 2;
    let cy = window.height / 2;

    let tlen = title.len();
    stdout
        .queue(MoveTo(cx - tlen as u16 / 2, cy - 3))?
        .queue(SetForegroundColor(palette.score_text))?
        .queue(SetAttribute(Attribute::Bold))?
        .queue(Print(title))?
        .queue(SetAttribute(Attribute::NoBold))?;

    let separator = MENU_SEP.to_string().repeat(tlen);
    stdout
        .queue(MoveTo(cx - tlen as u16 / 2, cy - 2))?
        .queue(SetForegroundColor(palette.score_text))?
        .queue(Print(separator))?;

    let keys_max = options
        .iter()
        .map(|(k, _)| k.chars().count())
        .max()
        .unwrap_or(0);

    for (i, (key, value)) in options.iter().enumerate() {
        let line = format!("{:^keys_max$}  {MENU_ARROW}  {:<}", key, value);
        stdout
            .queue(MoveTo(cx - (keys_max + 2) as u16, cy + i as u16))?
            .queue(SetForegroundColor(palette.score_text))?
            .queue(Print(&line))?;
    }

    stdout.queue(SetForegroundColor(Color::Reset))?;

    stdout.flush()
}
