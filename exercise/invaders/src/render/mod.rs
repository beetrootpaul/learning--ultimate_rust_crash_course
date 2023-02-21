use std::error::Error;
use std::io::{Stdout, Write};

use crossterm::cursor::MoveTo;
use crossterm::QueueableCommand;
use crossterm::style::{Color, SetBackgroundColor};
use crossterm::terminal::{Clear, ClearType};

use crate::frame;

pub fn render(stdout: &mut Stdout, last_frame: &frame::Frame, curr_frame: &frame::Frame, force: bool) -> Result<(), Box<dyn Error>> {
    if force {
        stdout.queue(SetBackgroundColor(Color::Blue))?;
        stdout.queue(Clear(ClearType::All))?;
        stdout.queue(SetBackgroundColor(Color::Black))?;
    }
    for (x, col) in curr_frame.iter().enumerate() {
        for (y, s) in col.iter().enumerate() {
            if *s != last_frame[x][y] || force {
                stdout.queue(MoveTo(x as u16, y as u16))?;
            }
        }
    }
    stdout.flush()?;

    Ok(())
}