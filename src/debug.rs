use bevy::prelude::*;
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};

use crate::npc::NPC;
use crate::player::Player;
use crate::ui::dialog_box::{DialogPanel, PlayerScroll, UpperScroll};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugin(WorldInspectorPlugin::new())
                .register_inspectable::<Player>()
                .register_inspectable::<NPC>()

                // UI

                .register_inspectable::<DialogPanel>()
                // .register_inspectable::<DialogBox>()
                .register_inspectable::<UpperScroll>()
                .register_inspectable::<PlayerScroll>()
                ;
        }
    }
}
