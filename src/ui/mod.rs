use bevy::{prelude::*, winit::WinitSettings};

mod dialog_box;
pub mod dialog_panel;
mod dialog_player;
pub mod dialog_scroll;
pub mod dialog_system;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    // #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        app
            // OPTIMIZE: Only run the app when there is user input. This will significantly reduce CPU/GPU use.
            .insert_resource(WinitSettings::game())
            .add_event::<dialog_panel::CreateDialogPanelEvent>()
            .add_event::<dialog_panel::CloseDialogPanelEvent>()
            .add_event::<dialog_panel::EndNodeDialogEvent>()
            .add_event::<dialog_scroll::UpdateScrollEvent>()
            .add_event::<dialog_player::DialogDiveEvent>()
            .add_event::<dialog_player::DropFirstTextUpperScroll>()
            .add_event::<dialog_box::ResetDialogBoxEvent>()
            // Trigger Event
            // .add_event::<dialog_system::FightEvent>()
            // .add_event::<dialog_system::TriggerEvent>()
            .add_systems(Startup, dialog_panel::load_textures)
            // OPTIMIZE: System Ordering
            .add_systems(
                Update,
                (
                    dialog_panel::create_dialog_panel_on_key_press,
                    dialog_panel::create_dialog_panel_on_combat_event,
                    dialog_panel::create_dialog_panel,
                    dialog_panel::update_dialog_panel,
                    dialog_panel::update_dialog_tree,
                    dialog_scroll::animate_scroll,
                    dialog_scroll::update_upper_scroll,
                    dialog_scroll::update_player_scroll,
                    dialog_box::reset_dialog_box,
                    dialog_box::update_dialog_box,
                    dialog_player::button_system,
                    dialog_player::hide_empty_button,
                    dialog_player::skip_forward_dialog,
                    dialog_player::dialog_dive,
                    dialog_player::drop_first_text_upper_scroll,
                    // crash when in this big tuple: (but not when in a simple `.add_systems()`)
                    // dialog_player::throw_trigger_event.after(dialog_player::dialog_dive),
                ),
            )
            // crash when in this big tuple: (but not when in a simple `.add_systems()`)
            .add_systems(Update, dialog_panel::end_node_dialog)
            .add_systems(Update, dialog_panel::close_dialog_panel)
            .add_systems(Update, dialog_panel::despawn_dialog_panel);
    }
}

#[derive(Component)]
pub struct UiElement;
