pub mod dialog_box;
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<dialog_box::CreateDialogBoxEvent>()
            .add_event::<dialog_box::CloseDialogBoxEvent>()
            .add_startup_system(dialog_box::load_textures)
            .add_system(dialog_box::update_dialog_box)
            .add_system(dialog_box::animate_scroll)
            .add_system(dialog_box::create_dialog_box)
            .add_system(dialog_box::close_dialog_box)
            .add_system(dialog_box::create_dialog_box_on_key_press)
            .add_system(dialog_box::despawn_dialog_box);
    }
}

#[derive(Component)]
pub struct UiElement;
