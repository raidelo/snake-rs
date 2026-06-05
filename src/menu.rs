use std::io::{self, Write, stdout};

use crossterm::{
    QueueableCommand,
    cursor::MoveTo,
    style::{Attribute, Color, Print, SetAttribute, SetBackgroundColor, SetForegroundColor},
};

use crate::constants::{MENU_ARROW, MENU_SEP};

pub fn menu(window: &super::Window, options: &[(&str, &str)]) -> Result<(), io::Error> {
    let mut stdout = stdout();

    let palette = &window.palette;
    let cx = window.width / 2;
    let cy = window.height / 2;

    let title = "S N A K E";
    let tlen = title.len();
    stdout
        .queue(MoveTo(cx - tlen as u16 / 2, cy - 3))?
        .queue(SetForegroundColor(palette.score_text))?
        .queue(SetAttribute(Attribute::Bold))?
        .queue(Print(title))?
        .queue(SetAttribute(Attribute::Reset))?
        .queue(SetBackgroundColor(palette.background))?;

    let separator = MENU_SEP.to_string().repeat(tlen);
    stdout
        .queue(MoveTo(cx - tlen as u16 / 2, cy - 2))?
        .queue(SetForegroundColor(palette.score_text))?
        .queue(Print(separator))?;

    let (mut key_max, _) = (0, 0);

    for (key, _) in options.iter() {
        let key_len = key.chars().count();
        if key_len > key_max {
            key_max = key_len;
        }
    }

    let mut first_line_align: u16 = 0;

    for (i, (key, label)) in options.iter().enumerate() {
        let line = format!("{:^key_max$}  {MENU_ARROW}  {:<}", key, label);
        if i == 0 {
            first_line_align = line.chars().count() as u16 / 2;
        }
        stdout
            .queue(MoveTo(cx - first_line_align, cy + i as u16))?
            .queue(SetForegroundColor(palette.score_text))?
            .queue(Print(&line))?;
    }

    stdout.queue(SetForegroundColor(Color::Reset))?.flush()?;

    stdout.flush()
}

pub enum MenuChoice {
    Start,
    Quit,
}

pub enum MenuError {
    IOError(io::Error),
    Interrupt,
}

impl From<io::Error> for MenuError {
    fn from(value: io::Error) -> Self {
        Self::IOError(value)
    }
}
