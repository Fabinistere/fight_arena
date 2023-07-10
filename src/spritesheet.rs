use bevy::prelude::*;

pub struct FabienPlugin;

#[derive(Clone, Resource)]
pub struct FabienSheet(pub Handle<TextureAtlas>);

impl Plugin for FabienPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_character_spritesheet.in_base_set(StartupSet::PreStartup));
    }
}

fn load_character_spritesheet(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let image = assets.load("textures/character/big_sprite_sheet.png");
    let atlas = TextureAtlas::from_grid(image, Vec2::splat(34.), 4, 12, None, None);

    let atlas_handle = texture_atlases.add(atlas);

    commands.insert_resource(FabienSheet(atlas_handle));
}
