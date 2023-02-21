use std::{io, thread, time};
use std::error::Error;
use std::sync::mpsc;

use crossterm::ExecutableCommand;

use invaders::{frame, render};

fn main() -> Result<(), Box<dyn Error>> {
    let (frame_sender, frame_receiver, mut audio) = setup().expect("should setup");

    audio.play("startup");

    let render_handler = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true).expect("should render");
        loop {
            let curr_frame = match frame_receiver.recv() {
                Ok(frame) => frame,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false).expect("should render");
            last_frame = curr_frame;
        }
    });

    'game_loop: loop {
        let curr_frame = frame::new_frame();
        while crossterm::event::poll(time::Duration::default())? {
            if let crossterm::event::Event::Key(key_event) = crossterm::event::read()? {
                match key_event.code {
                    crossterm::event::KeyCode::Esc | crossterm::event::KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'game_loop;
                    }
                    _ => {}
                };
            }
        }

        frame_sender.send(curr_frame)?;
        // TODO: tet other millis
        thread::sleep(time::Duration::from_millis(1));
    }

    drop(frame_sender);
    clean_up(audio, render_handler).expect("should clean up");
    Ok(())
}

fn setup() -> Result<(mpsc::Sender<frame::Frame>, mpsc::Receiver<frame::Frame>, rusty_audio::Audio), Box<dyn Error>> {
    let (frame_sender, frame_receiver) = mpsc::channel();

    let mut audio = rusty_audio::Audio::new();

    audio.add("explode", "assets/sounds/explode.wav");
    audio.add("lose", "assets/sounds/lose.wav");
    audio.add("move", "assets/sounds/move.wav");
    audio.add("pew", "assets/sounds/pew.wav");
    audio.add("startup", "assets/sounds/startup.wav");
    audio.add("win", "assets/sounds/win.wav");

    let mut stdout = io::stdout();
    crossterm::terminal::enable_raw_mode()?;
    stdout.execute(crossterm::terminal::EnterAlternateScreen)?;
    stdout.execute(crossterm::cursor::Hide)?;

    Ok((frame_sender, frame_receiver, audio))
}

fn clean_up(audio: rusty_audio::Audio, render_handler: thread::JoinHandle<()>) -> Result<(), Box<dyn Error>> {
    audio.wait();

    render_handler.join().unwrap();

    let mut stdout = io::stdout();
    stdout.execute(crossterm::cursor::Show)?;
    stdout.execute(crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}