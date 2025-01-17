use std::io::{Stdout, Write};
use crossterm::cursor::MoveTo;

use crossterm::style::{Color, SetBackgroundColor};
use crossterm::terminal::{Clear, ClearType};
use crossterm::QueueableCommand;

use crate::frame::Frame;
use crate::helpers::ResultAnyErr;

pub fn render(
    stdout: &mut Stdout,
    last_frame: &Frame,
    curr_frame: &Frame,
    force: bool,
) -> ResultAnyErr<()> {
    if force {
        stdout.queue(SetBackgroundColor(Color::Blue))?;
        stdout.queue(Clear(ClearType::All))?;
        stdout.queue(SetBackgroundColor(Color::Black))?;
    }

    for (x, col) in curr_frame.iter().enumerate() {
        for (y, s) in col.iter().enumerate() {
            if *s != last_frame[x][y] || force {
                stdout.queue(MoveTo(x as u16, y as u16))?;
                print!("{}", *s);
            }
        }
    }

    stdout.flush()?;

    Ok(())
}
