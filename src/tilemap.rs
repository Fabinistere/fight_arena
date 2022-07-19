use bevy::prelude::*;

use crate::{FabienSheet, TILE_SIZE};

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(
                StartupStage::PreStartup,
                load_simple_map
            );
    }
}

fn load_simple_map(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
)
{
    let image = assets.load("SimpleTemple.png");
    let atlas = TextureAtlas::from_grid(
        image,
        Vec2::splat(300.0),
        1,
        1,
    );
    let atlas_handle
        = texture_atlases.add(atlas);

        commands.insert_resource(FabienSheet(atlas_handle));
}