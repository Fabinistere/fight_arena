use bevy::prelude::*;

pub struct FabienPlugin;

#[derive(Clone)]
pub struct FabienSheet(pub Handle<TextureAtlas>);

impl Plugin  for FabienPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(
                StartupStage::PreStartup,
                load_character_spritesheet
            );

    }
}

fn load_character_spritesheet(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
)
{
    let image = assets.load("textures/character/big_sprite_sheet.png");
    let atlas = TextureAtlas::from_grid(
        image,
        Vec2::splat(34.),
        4,
        12,
    );

    let atlas_handle
        = texture_atlases.add(atlas);
    
    commands.insert_resource(FabienSheet(atlas_handle));
}