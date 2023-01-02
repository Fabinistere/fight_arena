use bevy::{prelude::*, winit::WinitSettings};

pub mod dialog_box;
pub mod dialog_player;
pub mod dialog_system;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            // OPTIMIZE: Only run the app when there is user input. This will significantly reduce CPU/GPU use.
            .insert_resource(WinitSettings::game())

            .add_event::<dialog_box::CreateDialogBoxEvent>()
            .add_event::<dialog_box::CloseDialogBoxEvent>()
            .add_event::<dialog_box::UpdateScrollEvent>()
            .add_event::<dialog_box::ResetDialogBoxEvent>()
            .add_event::<dialog_box::EndNodeDialogEvent>()
            .add_event::<dialog_player::DialogDiveEvent>()
            .add_event::<dialog_player::DropFirstTextUpperScroll>()

            .add_startup_system(dialog_box::load_textures)

            .add_system(dialog_box::create_dialog_box_on_key_press)
            .add_system(dialog_box::create_dialog_box_on_combat_event)
            
            .add_system(dialog_box::create_dialog_box)
            .add_system(dialog_box::animate_scroll)

            .add_system(dialog_box::update_dialog_panel)
            .add_system(dialog_box::update_dialog_tree)
            .add_system(dialog_box::update_upper_scroll)
            .add_system(dialog_box::update_player_scroll)
            .add_system(dialog_box::reset_dialog_box)
            .add_system(dialog_box::update_dialog_box)
            
            .add_system(dialog_player::button_system)
            .add_system(dialog_player::hide_empty_button)
            .add_system(dialog_player::skip_forward_dialog)

            .add_system(dialog_player::dialog_dive)
            .add_system(dialog_player::drop_first_text_upper_scroll)

            .add_system(dialog_box::end_node_dialog)
            .add_system(dialog_box::close_dialog_box)
            .add_system(dialog_box::despawn_dialog_box);
    }
}

#[derive(Component)]
pub struct UiElement;
