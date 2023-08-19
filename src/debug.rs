use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::{
    collisions::{TesselatedCollider, TesselatedColliderConfig},
    ui::{
        dialog_panel::DialogPanel,
        dialog_scroll::{PlayerScroll, UpperScroll},
    },
};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugins((
                WorldInspectorPlugin::new(),
            ))

                /* -------------------------------------------------------------------------- */
                /*                                     UI                                     */
                /* -------------------------------------------------------------------------- */

                .register_type::<DialogPanel>()
                // .register_type::<DialogBox>()
                .register_type::<UpperScroll>()
                .register_type::<PlayerScroll>()

                /* -------------------------------------------------------------------------- */
                /*                                   Hitbox                                   */
                /* -------------------------------------------------------------------------- */

                .register_type::<TesselatedCollider>()
                .register_type::<TesselatedColliderConfig>()
                ;
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
