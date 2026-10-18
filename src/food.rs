use bevy::prelude::Component;
use rand::prelude::*;

const FOODS: [(&str, u32); 3] = [
    ("apple.png", 2),
    ("banana.png", 5),
    ("watermelon.png", 8),
];

#[derive(Component)]
pub(crate) struct Food {
    pub value: u32,
    pub asset: String,
}

impl Food {
    pub fn random() -> Food {
        let mut rng = rand::rng();
        let kind: usize = rng.random_range(0..FOODS.len());
        Food { value: FOODS[kind].1, asset: FOODS[kind].0.to_string() }
    }
}
