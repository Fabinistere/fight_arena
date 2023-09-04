use bevy::prelude::*;
use bevy_inspector_egui::quick::{
    ResourceInspectorPlugin, StateInspectorPlugin, WorldInspectorPlugin,
};

use crate::{
    collisions::{TesselatedCollider, TesselatedColliderConfig},
    ui::{
        dialog_scrolls::Monolog,
        dialog_systems::{ActiveWorldEvents, CurrentInterlocutor},
    },
    GameState,
};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugins((WorldInspectorPlugin::new(),))
                .register_type::<GameState>()
                .register_type::<Monolog>()
                .register_type::<CurrentInterlocutor>()
                .register_type::<ActiveWorldEvents>()
                .add_plugins((
                    StateInspectorPlugin::<GameState>::default(),
                    ResourceInspectorPlugin::<Monolog>::default(),
                    ResourceInspectorPlugin::<CurrentInterlocutor>::default(),
                    ResourceInspectorPlugin::<ActiveWorldEvents>::default(),
                ))
                /* -------------------------------------------------------------------------- */
                /*                                     UI                                     */
                /* -------------------------------------------------------------------------- */
                // .register_type::<DialogBox>()
                /* -------------------------------------------------------------------------- */
                /*                                   Hitbox                                   */
                /* -------------------------------------------------------------------------- */
                .register_type::<TesselatedCollider>()
                .register_type::<TesselatedColliderConfig>();
        }
    }
}

// TODO: Create debug log kind
// Combat Debug
// Movement Debug
// Dialog Debug
// ...

// make it clear in the global log (different files ?)
//   - global log file
//   - specific (Combat/Movement/Dialog) log file
// ask for sending logs and data to *me* when game crash

// TODO: Create Custom Lint Rule
// function using query not being added to a plugin
// event ...
// plugin ...

// TODO: Create Contribution Example
// for
// - fn
// - struct
//   - Component
//   - Event
//   - Plugin
// - Module
