use bevy::prelude::*;

use crate::common::Direction;

#[derive(Component)]
pub(crate) struct SnakeBodyPiece;

impl SnakeBodyPiece {
    pub const BODY_COLOR: Color = Color::rgb(0.6, 0.6, 0.6);

    pub fn new() -> SnakeBodyPiece {
        SnakeBodyPiece {}
    }
}

#[derive(Component)]
pub(crate) struct SnakeHead {
    pub direction: Direction,
    pub next_turn: bool,
    growth: u32,
    body: Vec<Entity>,
}

impl SnakeHead {
    pub const HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);

    pub fn new() -> Self {
        SnakeHead {
            direction: Direction::Left,
            next_turn: false,
            growth: 0,
            body: Vec::new(),
        }
    }

    pub fn turn(&mut self, direction: Direction) {
        if SnakeHead::get_opposite_direction(direction) != self.direction {
            self.direction = direction;
            self.next_turn = true;
        }
    }

    pub fn grow(&mut self, value: u32) {
        self.growth += value;
    }

    pub fn can_grow(&self) -> bool {
        if self.growth > 0 {
            true
        }
        else {
            false
        }
    }

    pub fn add_body_piece(&mut self, entity: Entity) {
        if self.growth > 0 {
            self.growth -= 1;
        }
        println!("Adding body piece: {:?}", entity);
        self.body.push(entity);
    }

    pub fn get_last_body_piece(&self) -> Option<Entity> {
        match self.body.len() {
            0 => None,
            n => Some(self.body[n-1])
        }
    }

    pub fn move_last_body_piece_to_front(&mut self) {
        let len = self.body.len();
        if len > 1 {
            self.body.insert(0, self.body[len-1]);
            self.body.pop();
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
