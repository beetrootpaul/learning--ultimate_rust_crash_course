pub enum Sounds {
    Explode,
    Lose,
    Move,
    Pew,
    Startup,
    Win,
}

impl Sounds {
    pub fn name(&self) -> &str {
        match *self {
            Sounds::Explode => "explode",
            Sounds::Lose => "lose",
            Sounds::Move => "move",
            Sounds::Pew => "pew",
            Sounds::Startup => "startup",
            Sounds::Win => "win",
        }
    }
    pub fn path(&self) -> &str {
        match *self {
            Sounds::Explode => "assets/sounds/explode.wav",
            Sounds::Lose => "assets/sounds/lose.wav",
            Sounds::Move => "assets/sounds/move.wav",
            Sounds::Pew => "assets/sounds/pew.wav",
            Sounds::Startup => "assets/sounds/startup.wav",
            Sounds::Win => "assets/sounds/win.wav",
        }
    }
}
