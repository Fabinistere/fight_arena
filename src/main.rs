//! Fight Arena is a test repertory where I can develop the NPC and Combat system for our FTO game
//!
//! Follow along with the tutorial serie by Logic Projects <3

#![allow(clippy::redundant_field_names)]
use bevy::{
    prelude::*,
    render::{camera::ScalingMode, texture::ImagePlugin},
};
use bevy_rapier2d::prelude::*;
use bevy_tweening::TweeningPlugin;

pub mod collisions;
mod combat;
pub mod constants;
mod debug;
mod locations;
mod movement;
mod npc;
pub mod player;
mod spritesheet;
pub mod ui;

use collisions::RetroPhysicsPlugin;
use combat::CombatPlugin;
use constants::*;
use debug::DebugPlugin;
use locations::LocationsPlugin;
use npc::NPCPlugin;
use player::PlayerPlugin;
use spritesheet::{FabienPlugin, FabienSheet};
use ui::UiPlugin;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Menu,
    Playing,
    Interaction,
    Combat,
    Discussion,
}

#[rustfmt::skip]
fn main() {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();
    
    let height = 720.0;

    let mut app = App::new();
    app

        .insert_resource(ClearColor(CLEAR))
        .insert_resource(Msaa { samples: 1 })
        // hitbox
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })

        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        width: height * RESOLUTION,
                        height,
                        title: "Fight Arena".to_string(),
                        resizable: false,
                        ..default()
                    },
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(RapierDebugRenderPlugin {
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
        .add_startup_system(music)
        ;

    app.run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.top = 50. * TILE_SIZE;
    camera.projection.bottom = -50. * TILE_SIZE;

    camera.projection.left = 50. * TILE_SIZE * RESOLUTION;
    camera.projection.right = -50. * TILE_SIZE * RESOLUTION;

    camera.projection.scaling_mode = ScalingMode::None;

    commands.spawn(camera);
}

fn music(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    let music = asset_server.load("sounds/FTO_Dracula_theme.ogg");
    // audio.play(music);
    audio.play_with_settings(music, PlaybackSettings::LOOP.with_volume(0.10));

    println!("audio playing...");
}
