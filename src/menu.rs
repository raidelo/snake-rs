use std::io::{self, Write, stdout};

use crossterm::{
    QueueableCommand,
    cursor::MoveTo,
    style::{Attribute, Color, Print, SetAttribute, SetBackgroundColor, SetForegroundColor},
};

use crate::{
    constants::{ARROW_GLYPH, SEP_GLYPH},
    types::Window,
};

pub fn menu(window: &Window, title: &str, options: &[(&str, &str)]) -> Result<(), io::Error> {
    let mut stdout = stdout();

    let palette = &window.palette;
    let cx = window.width / 2;
    let cy = window.height / 2;

    let tlen = title.chars().count();
    let title_x = cx.saturating_sub(tlen as u16 / 2);

    stdout
        .queue(SetBackgroundColor(palette.background))?
        .queue(SetForegroundColor(palette.score_text))?
        .queue(MoveTo(title_x, cy - 3))?
        .queue(SetAttribute(Attribute::Bold))?
        .queue(Print(title))?
        .queue(SetAttribute(Attribute::NoBold))?;

    let separator = SEP_GLYPH.to_string().repeat(tlen);
    stdout
        .queue(MoveTo(title_x, cy - 2))?
        .queue(Print(separator))?;

    let keys_max_len = options
        .iter()
        .map(|(k, _)| k.chars().count())
        .max()
        .unwrap_or(0);

    let rows: Vec<String> = options
        .iter()
        .map(|(key, value)| format!("{:^keys_max_len$}  {ARROW_GLYPH}  {}", key, value))
        .collect();

    let rows_max_len = rows
        .iter()
        .map(|row| row.chars().count())
        .max()
        .unwrap_or(0);

    let rows_x = cx.saturating_sub((rows_max_len / 2) as u16);

    for (i, row) in rows.iter().enumerate() {
        stdout
            .queue(MoveTo(rows_x, cy + i as u16))?
            .queue(Print(row))?;
    }

    stdout
        .queue(SetForegroundColor(Color::Reset))?
        .queue(SetBackgroundColor(Color::Reset))?;

    stdout.flush()
}
