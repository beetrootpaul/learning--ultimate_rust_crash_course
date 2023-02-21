use std::error::Error;
use std::io;
use std::io::Stdout;
use crossterm::cursor::{Hide, Show};
use crossterm::{ExecutableCommand, terminal};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};

use rusty_audio::Audio;

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    let mut stdout = io::stdout();
    setup(&mut audio, &mut stdout).expect("should setup");

    audio.play("startup");

    clean_up(audio, stdout).expect("should clean up");
    Ok(())
}

// fn setup(mut audio: &Audio, mut stdout: &Stdout) -> Result<(), Box<dyn Error>> {
fn setup(audio: &mut Audio, stdout: &mut Stdout) -> Result<(), Box<dyn Error>> {
    audio.add("explode", "assets/sounds/explode.wav");
    audio.add("lose", "assets/sounds/lose.wav");
    audio.add("move", "assets/sounds/move.wav");
    audio.add("pew", "assets/sounds/pew.wav");
    audio.add("startup", "assets/sounds/startup.wav");
    audio.add("win", "assets/sounds/win.wav");

    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    Ok(())
}

// fn cleanup(mut audio: Audio, mut stdout: Stdout) -> Result<(), Box<dyn Error>>  {
fn clean_up(audio: Audio, mut stdout: Stdout) -> Result<(), Box<dyn Error>>  {

    audio.wait();

    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}