use bevy::{ecs::schedule::ShouldRun, prelude::*};

// use crate::{FabienSheet, TILE_SIZE};
use crate::{constants::locations::temple::*, player::Player, GameState};
use super::{spawn_collision_cuboid, Location};

pub struct TemplePlugin;

impl Plugin for TemplePlugin {
    fn build(&self, app: &mut App) {
        app .add_state(PlayerLocation::Temple)
            .add_system_set(
                SystemSet::on_enter(Location::Temple)
                    .with_system(setup_temple)
                    // .with_system(spawn_hitboxes)
            )
            .add_system_set_to_stage(
                CoreStage::PostUpdate,
                SystemSet::new()
                    .with_run_criteria(run_if_in_temple)
                    .with_system(throne_position),
            );
    }
}

#[derive(Component)]
pub struct Temple;
#[derive(Component)]
struct Throne;
#[derive(Component, Deref, DerefMut)]
pub struct ZPosition(f32);

// States
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum PlayerLocation {
    Temple,
}

fn run_if_in_temple(
    location: Res<State<Location>>,
    game_state: Res<State<GameState>>,
) -> ShouldRun {
    if location.current() == &Location::Temple && game_state.current() == &GameState::Playing {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

fn throne_position(
    player_query: Query<&GlobalTransform, With<Player>>,
    mut throne_query: Query<&mut Transform, With<Throne>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for mut throne_transform in throne_query.iter_mut() {
            if player_transform.translation.y > throne_transform.translation.y {
                throne_transform.translation.z = THRONE_Z_FRONT;
            } else {
                throne_transform.translation.z = THRONE_Z_BACK;
            }
        }
    }
}

// Spawns all entity related to the temple
fn setup_temple(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    // let background = asset_server.load("textures/temple/background.png");
    let main_room = asset_server.load("textures/temple/main_room.png");
    // let pillar = asset_server.load("textures/temple/pillar.png");
    let throne = asset_server.load("textures/temple/throne.png");
    // let corridor_doors = asset_server.load("textures/temple/corridor_doors.png");

    // All the temple sprites
    
    // commands.spawn_bundle(SpriteBundle {
    //     texture: background,
    //     transform: Transform::from_xyz(0.0, 0.0, BACKGROUND_Z),
    //     ..SpriteBundle::default()
    // });

    commands
        .spawn_bundle(SpriteBundle {
            texture: main_room,
            transform: Transform::from_xyz(0.0, 0.0, TEMPLE_Z),
            ..SpriteBundle::default()
        })
        .insert(Temple);

    commands
        .spawn_bundle(SpriteBundle {
            texture: throne,
            transform: Transform::from_translation(THRONE_POSITION.into()),
            ..SpriteBundle::default()
        })
        .insert(Throne);

    // commands.spawn_bundle(SpriteBundle {
    //     texture: corridor_doors,
    //     transform: Transform::from_xyz(0.0, 0.0, CORRIDOR_DOORS_Z),
    //     ..SpriteBundle::default()
    // });

    // for pos in PILLAR_POSITIONS {
    //     commands
    //         .spawn_bundle(SpriteBundle {
    //             texture: pillar.clone(),
    //             transform: Transform::from_translation(pos.into()),
    //             ..SpriteBundle::default()
    //         })
    //         .insert(Pillar)
    //         .with_children(|parent| {
    //             parent
    //                 .spawn()
    //                 .insert(Collider::cuboid(60.0, 20.0))
    //                 .insert(Transform::from_xyz(pos.0, pos.1 - 110.0, 0.0));
    //         });
    // }
}


fn spawn_hitboxes(mut commands: Commands) {
    // Left wall
    spawn_collision_cuboid(&mut commands, -1320.0, 80.0, 10.0, 1455.0);
    // Right wall
    spawn_collision_cuboid(&mut commands, 860.0, 80.0, 10.0, 1455.0);
    // Left side of top wall
    spawn_collision_cuboid(&mut commands, -895.0, 975.0, 415.0, 30.0);
    // Right side of top wall
    spawn_collision_cuboid(&mut commands, 225.0, 975.0, 625.0, 30.0);
    // Left side of bottom wall
    spawn_collision_cuboid(&mut commands, -815.0, -805.0, 495.0, 30.0);
    // Right side of bottom wall
    spawn_collision_cuboid(&mut commands, 355.0, -805.0, 495.0, 30.0);
    // Throne seat
    spawn_collision_cuboid(&mut commands, -230.0, 860.0, 70.0, 40.0);
    // Throne front of seat
    spawn_collision_cuboid(&mut commands, -230.0, 810.0, 50.0, 10.0);
    // Throne front of front of seat
    spawn_collision_cuboid(&mut commands, -230.0, 790.0, 30.0, 10.0);
    // Throne bump left 1
    spawn_collision_cuboid(&mut commands, -560.0, 875.0, 1.0, 60.0);
    // Throne bump right 1
    spawn_collision_cuboid(&mut commands, 100.0, 875.0, 1.0, 60.0);
    // Throne bump left 2
    spawn_collision_cuboid(&mut commands, -540.0, 785.0, 1.0, 30.0);
    // Throne bump right 2
    spawn_collision_cuboid(&mut commands, 80.0, 785.0, 1.0, 30.0);
    // Throne bump left 3
    spawn_collision_cuboid(&mut commands, -520.0, 710.0, 1.0, 45.0);
    // Throne bump right 3
    spawn_collision_cuboid(&mut commands, 60.0, 710.0, 1.0, 45.0);
    // Throne bump left 4
    spawn_collision_cuboid(&mut commands, -460.0, 635.0, 1.0, 30.0);
    // Throne bump right 4
    spawn_collision_cuboid(&mut commands, 0.0, 635.0, 1.0, 30.0);
}
