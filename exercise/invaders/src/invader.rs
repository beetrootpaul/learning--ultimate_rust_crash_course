use std::cmp;
use std::time::Duration;

use crate::frame::{Drawable, Frame};
use crate::{NUM_COLS, NUM_ROWS};

pub struct Invader {
    x: usize,
    y: usize,
}

pub struct Invaders {
    army: Vec<Invader>,
    move_timer: rusty_time::Timer,
    direction: Direction,
}

enum Direction {
    Left,
    Right,
}

impl Invaders {
    pub fn new() -> Self {
        let mut army = vec![];
        for x in 0..NUM_COLS {
            for y in 0..NUM_ROWS {
                if x > 1 && x < NUM_COLS - 2 && y > 0 && y < 9 && x % 2 == 0 && y % 2 == 0 {
                    army.push(Invader { x, y })
                }
            }
        }
        Self {
            army,
            move_timer: rusty_time::Timer::from_millis(2000),
            direction: Direction::Right,
        }
    }
    pub fn update(&mut self, delta: Duration) -> bool {
        self.move_timer.update(delta);
        if !(self.move_timer.ready) {
            return false;
        }
        self.move_timer.reset();

        let mut downwards = false;

        match self.direction {
            Direction::Left => {
                let min_x = self.army.iter().map(|invader| invader.x).min().unwrap_or(0);
                if min_x == 0 {
                    self.direction = Direction::Right;
                    downwards = true
                }
            }
            Direction::Right => {
                let max_x = self.army.iter().map(|invader| invader.x).max().unwrap_or(0);
                if max_x == NUM_COLS - 1 {
                    self.direction = Direction::Left;
                    downwards = true
                }
            }
        }

        if downwards {
            let new_duration = cmp::max(self.move_timer.duration.as_millis() - 250, 250);
            self.move_timer = rusty_time::Timer::from_millis(new_duration as u64);
            for invader in self.army.iter_mut() {
                invader.y += 1;
            }
        } else {
            for invader in self.army.iter_mut() {
                match self.direction {
                    Direction::Left => invader.x -= 1,
                    Direction::Right => invader.x += 1,
                }
            }
        }

        true
    }

    pub fn all_killed(&self) -> bool {
        self.army.is_empty()
    }

    pub fn reached_bottom(&self) -> bool {
        self.army.iter().map(|invader| invader.y).max().unwrap_or(0) >= NUM_ROWS - 1
    }

    pub fn kill_invader_at(&mut self, x: usize, y: usize) -> bool {
        if let Some(idx) = self
            .army
            .iter()
            .position(|invader| invader.x == x && invader.y == y)
        {
            self.army.remove(idx);
            true
        } else {
            false
        }
    }
}

impl Drawable for Invaders {
    fn draw(&self, frame: &mut Frame) {
        for invader in self.army.iter() {
            let animation_phase =
                self.move_timer.time_left.as_secs_f32() / self.move_timer.duration.as_secs_f32();
            frame[invader.x][invader.y] = if animation_phase > 0.5 { "x" } else { "+" };
        }
    }
}
