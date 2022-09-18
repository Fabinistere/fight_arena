//! Fight Arena is a test repertory where I can develop the NPC and Combat system for our FTO game
//! 
//! Follow along with the tutorial serie by Logic Projects <3

#![allow(clippy::redundant_field_names)]
use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_rapier2d::prelude::*;

pub mod constants;
mod combat;
mod debug;
mod spritesheet;
mod locations;
mod movement;
mod npc;
pub mod player;

use combat::CombatPlugin;
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
    Combat,
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
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
        .add_plugin(DebugPlugin)
        .add_plugin(FabienPlugin)
        .add_plugin(LocationsPlugin)
        .add_plugin(NPCPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(CombatPlugin)
        .add_state(GameState::Playing)
        .add_startup_system(spawn_camera)
        .add_startup_system(setup)
        ;

    app.run();
}

fn spawn_camera(
    mut commands: Commands
) {
    let mut camera = Camera2dBundle::default();

    camera.projection.top = 50.;
    camera.projection.bottom = -50.;

    camera.projection.left = 50. * RESOLUTION;
    camera.projection.right = -50. * RESOLUTION;

    camera.projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);

}

fn setup(
    asset_server: Res<AssetServer>, audio: Res<Audio>
) {
    let music = asset_server.load("sounds/FTO_Dracula_theme.ogg");
    audio.play(music);
    // audio.play_with_settings(music, PlaybackSettings::LOOP.with_volume(0.10));

    println!("audio playing...");
}