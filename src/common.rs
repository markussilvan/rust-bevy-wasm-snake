use bevy::prelude::{Component, Res, State, Deref, DerefMut, Timer};

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

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down
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
            Direction::Left => self.x -= length,
            Direction::Right => self.x += length,
            Direction::Up => self.y += length,
            Direction::Down => self.y -= length,
        }
    }
}

impl std::fmt::Display for GridPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn in_expected_state(state: Res<State<AppState>>, expected: AppState) -> bool {
    if *state.current() == expected {
        true
    }
    else {
        false
    }
}

pub fn convert_to_screen_coordinates(position: GridPosition) -> (f32, f32) {
    let x: f32 = position.x as f32 * GRID_SIZE - (WINDOW_WIDTH / 2.0);
    let y: f32 = position.y as f32 * GRID_SIZE - (WINDOW_HEIGHT / 2.0);
    (x, y)
}
