// Follow along with the tutorial serie by Logic Projects <3

#![allow(clippy::redundant_field_names)]
use bevy::{prelude::*, render::camera::ScalingMode};
// use bevy_rapier2d::prelude::*;

pub mod constants;
mod debug;
mod fabien;
mod locations;
pub mod player;
mod npc;


use debug::DebugPlugin;
use fabien::{FabienPlugin, FabienSheet, spawn_fabien_sprite};
use locations::LocationsPlugin;
use npc::NPCPlugin;
use player::PlayerPlugin;


pub use crate::{
    constants::*,
};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Menu,
    Playing,
}

fn main() {
    let height = 720.0;

    let mut app = App::new();
    app
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: height * RESOLUTION,
            height: height,
            title: "Fight Arena".to_string(),
            resizable: false,
            ..WindowDescriptor::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugPlugin)
        .add_plugin(FabienPlugin)
        .add_plugin(LocationsPlugin)
        .add_plugin(NPCPlugin)
        .add_plugin(PlayerPlugin)
        .add_state(GameState::Playing)
        .add_startup_system(spawn_camera);

    app.run();
}

fn spawn_camera(
    mut commands: Commands
) {
    let mut camera = OrthographicCameraBundle::new_2d();

    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;

    camera.orthographic_projection.left = 1.0 * RESOLUTION;
    camera.orthographic_projection.right = -1.0 * RESOLUTION;

    camera.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);
}