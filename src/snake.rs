use bevy::prelude::*;

use crate::common::Direction;

#[derive(Component)]
pub(crate) struct Snake {
    pub direction: Direction,
}

impl Snake {
    pub const HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);

    pub fn new() -> Self {
        Snake {
            direction: Direction::Left,
        }
    }

    pub fn turn(&mut self, direction: Direction) {
        if Snake::get_opposite_direction(direction) != self.direction {
            self.direction = direction;
        }
    }

    fn get_opposite_direction(direction: Direction) -> Direction {
        match direction {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }
}
