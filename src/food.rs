use bevy::prelude::Component;
use rand::prelude::*;

const FOODS: [(&str, u32); 2] = [
    ("apple.png", 1),
    ("banana.png", 5)
];

#[derive(Component)]
pub(crate) struct Food {
    pub value: u32,
    pub asset: String,
}

impl Food {
    pub fn random() -> Food {
        let mut rng = rand::thread_rng();
        let kind: usize = rng.gen::<usize>() % FOODS.len() as usize;
        Food { value: FOODS[kind].1, asset: FOODS[kind].0.to_string() }
    }
}
