use bevy::prelude::{Component, Deref, DerefMut, Timer, TimerMode};

use rand::prelude::*;

pub(crate) const GRID_WIDTH: u32 = 30;
pub(crate) const GRID_HEIGHT: u32 = 20;
pub(crate) const GRID_SIZE: f32 = 30.0;

pub(crate) const WINDOW_WIDTH: f32 = GRID_WIDTH as f32 * GRID_SIZE;
pub(crate) const WINDOW_HEIGHT: f32 = GRID_HEIGHT as f32 * GRID_SIZE;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    SplashScreen,
    Gameplay,
    GameOver
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct BackgroundImage;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct Text;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component, Deref, DerefMut, Default)]
pub struct DeathTimer {
    pub timer: Timer,
}

impl DeathTimer {
    pub fn default() -> Self {
        let seconds = 0.5;
        Self {
            timer: Timer::from_seconds(seconds, TimerMode::Once),
        }
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down
}

#[derive(Component, Clone, Copy)]
pub struct ScreenPosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<GridPosition> for ScreenPosition {
    fn from(position: GridPosition) -> Self {
        ScreenPosition {
            x: position.x as f32 * GRID_SIZE - (WINDOW_WIDTH / 2.0),
            y: position.y as f32 * GRID_SIZE - (WINDOW_HEIGHT / 2.0),
            z: 0.0,
        }
    }
}

impl std::fmt::Display for ScreenPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct GridPosition {
    pub x: u32,
    pub y: u32,
}

impl GridPosition {
    pub fn new(x: u32, y: u32) -> GridPosition {
        GridPosition { x, y }
    }

    pub fn random(max_x: u32, max_y: u32) -> GridPosition {
        let mut rng = rand::thread_rng();
        let x: i32 = rng.gen::<i32>() % max_x as i32;
        let y: i32 = rng.gen::<i32>() % max_y as i32;
        GridPosition { x: x.abs() as u32, y: y.abs() as u32}
    }

    pub fn move_position(&mut self, direction: Direction, length: u32) {
        match direction {
            Direction::Left => {
                if self.x >= length {
                    self.x -= length
                }
                else {
                    self.x = 0;
                }
            },
            Direction::Right => {
                if self.x + length < GRID_WIDTH {
                    self.x += length;
                }
                else {
                    self.x = GRID_WIDTH;
                }
            },
            Direction::Up => {
                if self.y + length < GRID_HEIGHT {
                    self.y += length;
                }
                else {
                    self.x = GRID_WIDTH;
                }
            },
            Direction::Down => {
                if self.y >= length {
                    self.y -= length;
                }
                else {
                    self.y = 0;
                }
            },
        }
    }
}

impl std::fmt::Display for GridPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn in_expected_state(state: &AppState, expected: AppState) -> bool {
    if *state == expected {
        true
    }
    else {
        false
    }
}
