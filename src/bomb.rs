use bevy::prelude::*;

#[derive(Component, Default)]
pub(crate) struct Bomb {
    pub timer: Timer,
    pub asset: String,
}

impl Bomb {
    pub fn default() -> Bomb {
        let seconds = 5.0;
        Bomb {
            timer: Timer::from_seconds(seconds, TimerMode::Once),
            asset: "bomb.png".to_string()
        }
    }
}
