use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable};

use crate::{FabienSheet, TILE_SIZE};
use crate::spawn_fabien_sprite;

pub struct PlayerPlugin;

#[derive(Component, Inspectable)]
pub struct Player{
    speed: f32
}

impl Plugin  for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_player)
            .add_system(camera_follow.after("movement"))
            .add_system(player_movement.label("movement"));

    }
}

fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>,
){
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
    
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
        4,
        Color::rgb(0.9,0.9,0.9),
        Vec3::new(0.0, 0.0, 5.0),
        Vec3::new(2.0, 2.0, 0.0)
    );

    commands
        .entity(player)
        .insert(Name::new("Player"))
        .insert(Player { speed: 3.0 });
}