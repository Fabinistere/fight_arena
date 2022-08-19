use bevy::prelude::*;

#[derive(Component)]
pub struct Speed(pub f32);

impl Default for Speed {
    fn default() -> Self {
        Speed (3.0)
    }
}

#[derive(Bundle)]
pub struct MovementBundle {
    pub speed: Speed,
}