//! Fight Arena is a test repertory where I can develop the NPC and Combat system for our FTO game
//! 
//! Follow along with the tutorial serie by Logic Projects <3

#![allow(clippy::redundant_field_names)]
use bevy::{
    prelude::*,
    render::{
        camera::ScalingMode,
        texture::ImageSettings
    }};
use bevy_rapier2d::prelude::*;
use bevy_tweening::TweeningPlugin;

pub mod collisions;
pub mod constants;
mod combat;
mod debug;
mod spritesheet;
mod locations;
mod movement;
mod npc;
pub mod player;
pub mod ui;

use collisions::RetroPhysicsPlugin;
use combat::CombatPlugin;
use constants::*;
use debug::DebugPlugin;
use spritesheet::{FabienPlugin, FabienSheet};
use locations::LocationsPlugin;
use npc::NPCPlugin;
use player::PlayerPlugin;
use ui::UiPlugin;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Menu,
    Playing,
    Interaction,
    Combat,
    Discussion
}

fn main() {
    let height = 720.0;

    let mut app = App::new();
    app

        .insert_resource(ClearColor(CLEAR))
        .insert_resource(ImageSettings::default_nearest())
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })
        .insert_resource(WindowDescriptor {
            width: height * RESOLUTION,
            height: height,
            title: "Fight Arena".to_string(),
            resizable: false,
            ..WindowDescriptor::default()
        })

        .add_plugins(DefaultPlugins)
        .add_plugin(RapierDebugRenderPlugin {
            depth_test: false,
            mode: DebugRenderMode::all(),
            ..default()
        })
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
        .add_plugin(RetroPhysicsPlugin::default())
        .add_plugin(TweeningPlugin)
        .add_plugin(CombatPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(FabienPlugin)
        .add_plugin(LocationsPlugin)
        .add_plugin(NPCPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(UiPlugin)

        .add_state(GameState::Playing)
        
        .add_startup_system(spawn_camera)
        // .add_startup_system(music)
        ;

    app.run();
}

fn spawn_camera(
    mut commands: Commands
) {
    let mut camera = Camera2dBundle::default();

    camera.projection.top = 50. * TILE_SIZE;
    camera.projection.bottom = -50. * TILE_SIZE;

    camera.projection.left = 50. * TILE_SIZE * RESOLUTION;
    camera.projection.right = -50. * TILE_SIZE * RESOLUTION;

    camera.projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);

}

fn music(
    asset_server: Res<AssetServer>, audio: Res<Audio>
) {

    
    let music = asset_server.load("sounds/FTO_Dracula_theme.ogg");
    // audio.play(music);
    audio.play_with_settings(music, PlaybackSettings::LOOP.with_volume(0.10));

    println!("audio playing...");
}