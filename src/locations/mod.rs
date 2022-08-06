use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod temple;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum Location {
    Temple,
}

pub struct LocationsPlugin;

impl Plugin for LocationsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(temple::TemplePlugin)
            .add_state(Location::Temple);
    }
}

pub fn spawn_collision_cuboid(
    commands: &mut Commands,
    x: f32,
    y: f32,
    width: f32,
    height: f32
) {
    commands
        .spawn()
        .insert(Collider::cuboid(width, height))
        .insert(Transform::from_xyz(x, y, 0.0))
        .insert(Friction::coefficient(0.0))
        .insert(Restitution::coefficient(0.0));
}
