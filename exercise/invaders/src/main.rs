use std::{io, thread, time};
use std::error::Error;
use std::sync::mpsc;
use std::time::Instant;

use crossterm::{event, ExecutableCommand};
use rusty_audio::Audio;

use invaders::assets::Sounds;
use invaders::frame::{Drawable, Frame, new_frame};
use invaders::helpers::ResultAnyErr;
use invaders::player::Player;
use invaders::render::render;

fn main() {
    let mut audio = setup().expect("should setup");

    game(&mut audio);

    clean_up(audio).expect("should clean up");
}

fn setup() -> Result<Audio, Box<dyn Error>> {
    let mut audio = Audio::new();

    audio.add(Sounds::Explode.name(), Sounds::Explode.path());
    audio.add(Sounds::Lose.name(), Sounds::Lose.path());
    audio.add(Sounds::Move.name(), Sounds::Move.path());
    audio.add(Sounds::Pew.name(), Sounds::Pew.path());
    audio.add(Sounds::Startup.name(), Sounds::Startup.path());
    audio.add(Sounds::Win.name(), Sounds::Win.path());

    let mut stdout = io::stdout();
    crossterm::terminal::enable_raw_mode()?;
    stdout.execute(crossterm::terminal::EnterAlternateScreen)?;
    stdout.execute(crossterm::cursor::Hide)?;

    Ok(audio)
}

fn clean_up(audio: Audio) -> ResultAnyErr<()> {
    audio.wait();

    let mut stdout = io::stdout();
    stdout.execute(crossterm::cursor::Show)?;
    stdout.execute(crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}

fn game(audio: &mut Audio) {
    let (frame_sender, frame_receiver): (mpsc::Sender<Frame>, mpsc::Receiver<Frame>) =
        mpsc::channel();

    let render_handler = thread::spawn(|| {
        render_loop(Box::new(move || frame_receiver.recv().ok()));
    });

    audio.play(Sounds::Startup.name());

    let player = Player::new();

    game_loop(
        audio,
        player,
        Box::new(move |curr_frame| {
            // just ignore the error, it's OK to have it fail to send some frames (e.g. when the app starts)
            frame_sender.send(curr_frame).unwrap_or(());
        }),
    );

    render_handler.join().unwrap();
}

fn render_loop(receive_frame: Box<dyn Fn() -> Option<Frame>>) {
    let mut stdout = io::stdout();
    let mut last_frame = new_frame();
    render(&mut stdout, &last_frame, &last_frame, true).expect("should render");
    while let Some(curr_frame) = receive_frame() {
        render(&mut stdout, &last_frame, &curr_frame, false).expect("should render");
        last_frame = curr_frame;
    }
}

fn game_loop(audio: &mut Audio, mut player: Player, send_frame: Box<dyn Fn(Frame)>) {
    let mut instant = Instant::now();
    'game_loop: loop {
        let delta = instant.elapsed();
        instant = Instant::now();

        let mut curr_frame = new_frame();

        while event::poll(time::Duration::default()).expect("should poll for events") {
            if let event::Event::Key(key_event) = event::read().expect("should read events") {
                match key_event.code {
                    event::KeyCode::Left => player.move_left(),
                    event::KeyCode::Right => player.move_right(),
                    event::KeyCode::Char(' ')
                    | event::KeyCode::Enter
                    | event::KeyCode::Char('z') => {
                        if player.shoot() {
                            audio.play(Sounds::Pew.name());
                        }
                    }
                    event::KeyCode::Esc | event::KeyCode::Char('q') => {
                        audio.play(Sounds::Lose.name());
                        break 'game_loop;
                    }
                    _ => {}
                };
            }
        }

        player.update(delta);

        player.draw(&mut curr_frame);

        send_frame(curr_frame);

        // let's wait a little bit to not generate way too many frames to be handled by a render loop
        thread::sleep(time::Duration::from_millis(10));
    }
}
