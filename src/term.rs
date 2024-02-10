use std::io;
use std::io::Stdout;

use crossterm::{
    cursor, queue,
    style::{self, Stylize},
};
pub struct Vec2 {
    pub x: u16,
    pub y: u16,
}

impl Vec2 {
    pub fn new(x: u16, y: u16) -> Self {
        Vec2 { x, y }
    }
}

pub fn write_box(
    stdout: &mut Stdout,
    color: style::Color,
    pos: Vec2,
    size: Vec2,
    title: Option<&str>,
) -> io::Result<()> {
    for y in 0..size.y {
        for x in 0..size.x {
            if x == 0 || x == size.x - 1 {
                queue!(
                    stdout,
                    cursor::MoveTo(pos.x + x, pos.y + y),
                    style::PrintStyledContent("|".with(color))
                )?;
            }
            if y == 0 || y == size.y - 1 {
                queue!(
                    stdout,
                    cursor::MoveTo(pos.x + x, pos.y + y),
                    style::PrintStyledContent("-".with(color))
                )?;
            }
        }
    }

    if let Some(title) = title {
        let text_padding = (size.x as f32 * 0.05f32) as u16;

        queue!(
            stdout,
            cursor::MoveTo(text_padding + pos.x, pos.y),
            style::PrintStyledContent(title.with(color))
        )?;
    }

    Ok(())
}
