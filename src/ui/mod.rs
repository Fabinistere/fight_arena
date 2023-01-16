use bevy::{prelude::*, winit::WinitSettings};

pub mod dialog_panel;
pub mod dialog_scroll;
mod dialog_player;
mod dialog_box;
pub mod dialog_system;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    #[rustfmt::skip]
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

            .add_startup_system(dialog_panel::load_textures)

            // OPTIMIZE: System Ordering
            
            .add_system(dialog_panel::create_dialog_panel_on_key_press)
            .add_system(dialog_panel::create_dialog_panel_on_combat_event)
            .add_system(dialog_panel::create_dialog_panel)
            
            .add_system(dialog_panel::update_dialog_panel)
            .add_system(dialog_panel::update_dialog_tree)

            .add_system(dialog_scroll::animate_scroll)

            .add_system(dialog_scroll::update_upper_scroll)
            .add_system(dialog_scroll::update_player_scroll)

            .add_system(dialog_box::reset_dialog_box)
            .add_system(dialog_box::update_dialog_box)

            .add_system(dialog_player::button_system)
            .add_system(dialog_player::hide_empty_button)
            .add_system(dialog_player::skip_forward_dialog)

            .add_system(dialog_player::dialog_dive)
            .add_system(dialog_player::drop_first_text_upper_scroll)

            .add_system(dialog_panel::end_node_dialog)
            .add_system(dialog_panel::close_dialog_panel)
            .add_system(dialog_panel::despawn_dialog_panel);
    }
}

#[derive(Component)]
pub struct UiElement;
