use bevy::prelude::*;
// use bevy_rapier2d::prelude::*;

mod temple;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, Reflect, States)]
enum Location {
    #[default]
    Temple,
}

pub struct LocationsPlugin;

impl Plugin for LocationsPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<Location>().add_plugin(temple::TemplePlugin);
    }
}
