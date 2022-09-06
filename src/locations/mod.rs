use bevy::prelude::*;
// use bevy_rapier2d::prelude::*;

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

