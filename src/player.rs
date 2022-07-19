use bevy::prelude::*;

// use bevy_inspector_egui::{Inspectable, RegisterInspectable};

use crate::{FabienSheet, TILE_SIZE};
use crate::spawn_fabien_sprite;

pub struct PlayerPlugin;

#[derive(Component)] // Inspectable
pub struct Player{
    speed: f32
}

impl Plugin  for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_player)
            .add_system(player_movement);

    }
}

fn player_movement(
    mut player_query: Query<(&mut Player, &mut Transform)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
){
    let (player, mut transform) = player_query.single_mut();

    if keyboard.pressed(KeyCode::Z) {
        transform.translation.y += player.speed * TILE_SIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::S) {
        transform.translation.y -= player.speed * TILE_SIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::Q) {
        transform.translation.x += player.speed * TILE_SIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::D) {
        transform.translation.x -= player.speed * TILE_SIZE * time.delta_seconds();
    }
}


fn spawn_player(
    mut commands: Commands,
    fabien: Res<FabienSheet>
)
{
    let player = spawn_fabien_sprite(
        &mut commands,
        &fabien,
        5,
        Color::None,
        Vec3::new(0.0, 0.0, 900.0)
    );
    let mut sprite = TextureAtlasSprite::new(1);
    //sprite.color = Color::rgb(0.3, 0.3, 0.9);
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE));

    commands
        .entity(player)
        .insert(Name::new("Player"))
        .insert(Player { speed: 3.0 });
}