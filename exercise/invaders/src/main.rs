use std::error::Error;
use std::io;
use std::io::Stdout;
use std::time::Duration;

use crossterm::{event, ExecutableCommand, terminal};
use crossterm::cursor::{Hide, Show};
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use rusty_audio::Audio;

fn main() -> Result<(), Box<dyn Error>> {
    let (mut audio, stdout) = setup().expect("should setup");

    audio.play("startup");

    'game_loop: loop {
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'game_loop;
                    }
                    _ => {}
                };
            }
        }
    }

    clean_up(audio, stdout).expect("should clean up");
    Ok(())
}

fn setup() -> Result<(Audio, Stdout), Box<dyn Error>> {
    let mut audio = Audio::new();
    let mut stdout = io::stdout();

    audio.add("explode", "assets/sounds/explode.wav");
    audio.add("lose", "assets/sounds/lose.wav");
    audio.add("move", "assets/sounds/move.wav");
    audio.add("pew", "assets/sounds/pew.wav");
    audio.add("startup", "assets/sounds/startup.wav");
    audio.add("win", "assets/sounds/win.wav");

    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    Ok((audio, stdout))
}

fn clean_up(audio: Audio, mut stdout: Stdout) -> Result<(), Box<dyn Error>> {
    audio.wait();

    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}