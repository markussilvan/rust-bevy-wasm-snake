use bevy::prelude::Component;

#[derive(Component)]
pub struct Wall {}

impl Wall {
    pub fn new() -> Wall {
        Wall {}
    }
}
