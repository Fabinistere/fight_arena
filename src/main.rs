//! Fight Arena is a test repertory where I can develop the NPC and Combat system for our FTO game
//!
//! Follow along with the tutorial serie by Logic Projects <3

#![allow(clippy::redundant_field_names)]
use bevy::{prelude::*, render::texture::ImagePlugin, window::WindowResolution};
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

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    /// TODO: feat Menu - put the default on Menu
    Menu,
    #[default]
    Playing,
    Interaction,
    Combat,
    Discussion,
}

// #[rustfmt::skip]
fn main() {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let mut app = App::new();
    app.add_state::<GameState>()
        .insert_resource(FixedTime::new_from_secs(FIXED_TIME_STEP))
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(Msaa::Off)
        // hitbox
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(HEIGHT * RESOLUTION, HEIGHT),
                        title: "Fight Arena".to_string(),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            RapierDebugRenderPlugin {
                mode: DebugRenderMode::all(),
                ..default()
            },
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.),
            RetroPhysicsPlugin::default(),
            TweeningPlugin,
            CombatPlugin,
            DebugPlugin,
            FabienPlugin,
            LocationsPlugin,
            NPCPlugin,
            PlayerPlugin,
            UiPlugin,
        )
        .add_systems(
            Startup,
            (
                spawn_camera, // music,
            ),
        );

    app.run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scale = 0.1;

    commands.spawn(camera);
}

fn _music(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    let music = asset_server.load("sounds/FTO_Dracula_theme.ogg");
    // audio.play(music);
    audio.play_with_settings(music, PlaybackSettings::LOOP.with_volume(0.10));

    println!("audio playing...");
}
