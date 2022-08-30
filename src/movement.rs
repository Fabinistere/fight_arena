use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

#[derive(Component, Deref, DerefMut)]
pub struct Speed(pub f32);

impl Default for Speed {
    fn default() -> Self {
        Speed (1.0)
    }
}

#[derive(Bundle)]
pub struct MovementBundle {
    pub speed: Speed,
    pub velocity: Velocity
}