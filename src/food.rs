use bevy::prelude::Component;

#[derive(Component)]
pub(crate) struct Food {
    pub value: u32,
}

impl Food {
    pub fn default() -> Food {
        Food { value: 1 }
    }
}
