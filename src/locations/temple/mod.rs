use bevy::{ecs::schedule::ShouldRun, prelude::*};
use bevy_rapier2d::prelude::*;
use bevy_retrograde::prelude::{TesselatedCollider, TesselatedColliderConfig};

// use crate::{FabienSheet, TILE_SIZE};
use crate::{
    constants::locations::temple::*,
    constants::player::PLAYER_Z,
    // constants::TILE_SIZE,
    GameState,
    player::Player,
};
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
    // let main_room = asset_server.load("textures/temple/main_room.png");
    // let throne = asset_server.load("textures/temple/throne.png");

    // let background = asset_server.load("textures/temple/bckgrd_credits.png");

    let banners = asset_server.load("textures/temple/temple_banners.png");
    let floor = asset_server.load("textures/temple/temple_floor.png");
    let wall = asset_server.load("textures/temple/temple_wall.png");
    // let light_off = asset_server.load("textures/temple/temple_light_off.png");
    // let light_on = asset_server.load("textures/temple/temple_light_on.png");
    // let museum = asset_server.load("textures/temple/temple_museum.png");
    let huge_throne = asset_server.load("textures/temple/temple_huge_throne.png");


    // All the temple sprites

    // let mut elements = Vec::new();

    commands
        .spawn_bundle(SpriteBundle {
            texture: banners.clone(),
            transform: Transform {
                translation: Vec3::new(0.23, 0.935, PLAYER_Z),
                scale: TEMPLE_SCALE.into(),
                ..default()
            },
            ..SpriteBundle::default()
        })
        .insert(RigidBody::Fixed)
        .insert(TesselatedCollider {
            texture: banners.clone(),
            // tesselator_config: TesselatedColliderConfig {
            //     // We want the collision shape for the banners to be highly accurate?
            //     vertice_separation: 0.,
            //     ..default()
            // },
            ..default()
        })
        .insert(Name::new("banners"));

    // elements.push(t_banners);

    commands
        .spawn_bundle(SpriteBundle {
            texture: wall.clone(),
            transform: Transform {
                translation: Vec3::new(0., 0., PLAYER_Z),
                scale: TEMPLE_SCALE.into(),
                ..default()
            },
            ..SpriteBundle::default()
        })
        .insert(RigidBody::Fixed)
        .insert(TesselatedCollider {
            texture: wall.clone(),
            // tesselator_config: TesselatedColliderConfig {
            //     vertice_separation: 0.,
            //     ..default()
            // },
            ..default()
        })
        .insert(Name::new("wall"));

    
    commands
        .spawn_bundle(SpriteBundle {
            texture: floor.clone(),
            transform: Transform {
                translation: TEMPLE_POSITION.into(),
                scale: TEMPLE_SCALE.into(),
                ..default()
            },
            ..SpriteBundle::default()
        })
        .insert(RigidBody::Fixed)
        .insert(Name::new("floor"));

    commands
        .spawn_bundle(SpriteBundle {
            texture: huge_throne.clone(),
            transform: Transform {
                translation: Vec3::new(0.23, 0.74, PLAYER_Z),
                scale: TEMPLE_SCALE.into(),
                ..default()
            },
            ..SpriteBundle::default()
        })
        .insert(RigidBody::Fixed)
        .insert(TesselatedCollider {
            texture: huge_throne.clone(),
            // tesselator_config: TesselatedColliderConfig {
            //     // We want the collision shape for the banners to be highly accurate?
            //     vertice_separation: 0.,
            //     ..default()
            // },
            ..default()
        })
        .insert(Name::new("throne"));

    // commands.spawn_bundle(SpriteBundle {
    //     texture: corridor_doors,
    //     transform: Transform::from_xyz(0.0, 0.0, CORRIDOR_DOORS_Z),
    //     ..SpriteBundle::default()
    // });
    
    // TODO to spawn pillar create a super layer on top of floor to visualize where it goes

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
    spawn_collision_cuboid(&mut commands, 0.01, 0.01, 10.0, 10.0);
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
