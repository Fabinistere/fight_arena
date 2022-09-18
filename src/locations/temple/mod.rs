use bevy::{ecs::schedule::ShouldRun, prelude::*};
use bevy_rapier2d::prelude::*;

use crate::{
    collisions::{TesselatedCollider, TesselatedColliderConfig},
    constants::{
        locations::temple::*,
        character::npc::{NPC_Z_BACK, NPC_Z_FRONT}
    },
    npc::NPC,
    player::Player
};
use super::Location;

pub struct TemplePlugin;

impl Plugin for TemplePlugin {
    fn build(&self, app: &mut App) {
        app .add_state(PlayerLocation::Temple)
            .add_system_set(
                SystemSet::on_enter(Location::Temple)
                    .with_system(setup_temple)
                    .with_system(spawn_pillars)
            )
            .add_system_set_to_stage(
                CoreStage::PostUpdate,
                SystemSet::new()
                    .with_run_criteria(run_if_in_temple)
                    .with_system(throne_position)
            )
            .add_system_set_to_stage(
                CoreStage::PostUpdate,
                SystemSet::new()
                    .with_run_criteria(run_if_in_temple)
                    .with_system(pillar_position)
            )
            .add_system_set_to_stage(
                CoreStage::PostUpdate,
                SystemSet::new()
                    .with_run_criteria(run_if_in_temple)
                    .with_system(npc_z_position)
            );
    }
}

#[derive(Component)]
pub struct Temple;

#[derive(Component)]
struct Throne;

#[derive(Component)]
struct Pillar;

#[derive(Component, Deref, DerefMut)]
pub struct ZPosition(f32);

// States
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum PlayerLocation {
    Temple,
}

fn run_if_in_temple(
    location: Res<State<Location>>,
) -> ShouldRun {
    if location.current() == &Location::Temple {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

/// doesn't work
fn npc_z_position(
    mut npc_query: Query<&mut Transform, With<NPC>>,
    pillar_query: Query<&GlobalTransform, With<Pillar>>,
) {
    // TODO prevent no transform in npc
    for mut npc_transform in npc_query.iter_mut() {
        for pillar_transform in pillar_query.iter() {
            // put the npc in front cause above the last pillar
            // the pb was: always below one pillar (the closest to the stage)
            // It only works when the npc was below the last pillar
            // this methods doesn't work cause we can be ABOVE and BELOW two diff pillars
            // between two line (in a single column)
            if npc_transform.translation.y <= pillar_transform.translation().y + 0.07
               &&
               npc_transform.translation.y >= pillar_transform.translation().y - 0.07 {
                if npc_transform.translation.y >= (pillar_transform.translation().y-PILLAR_ADJUST) {
                    npc_transform.translation.z = NPC_Z_BACK;
                } else {
                    npc_transform.translation.z = NPC_Z_FRONT;
                }
            }
            
        }
    }
}

fn throne_position(
    player_query: Query<&GlobalTransform, With<Player>>,
    mut throne_query: Query<&mut Transform, With<Throne>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for mut throne_transform in throne_query.iter_mut() {
            if player_transform.translation().y > throne_transform.translation.y {
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
    // elements.push(t_banners);

    commands
        .spawn_bundle(SpriteBundle {
            texture: wall.clone(),
            transform: Transform {
                translation: Vec3::new(0., 0., TEMPLE_Z),
                scale: TEMPLE_SCALE.into(),
                ..default()
            },
            ..SpriteBundle::default()
        })
        .insert(RigidBody::Fixed)
        // .insert(TesselatedCollider {
        //     texture: wall.clone(),
        //     ..default()
        // })
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
                translation: THRONE_POSITION.into(),
                scale: TEMPLE_SCALE.into(),
                ..default()
            },
            ..SpriteBundle::default()
        })
        .insert(RigidBody::Fixed)
        // .insert(TesselatedCollider {
        //     texture: huge_throne.clone(),
        //     tesselator_config: TesselatedColliderConfig {
        //         vertice_separation: 0.,
        //         ..default()
        //     },
        //     ..default()
        // })
        .insert(Throne)
        .insert(Name::new("throne"));

    commands
        .spawn_bundle(SpriteBundle {
            texture: banners.clone(),
            transform: Transform {
                translation: BANNERS_POSITION.into(),
                scale: TEMPLE_SCALE.into(),
                ..default()
            },
            ..SpriteBundle::default()
        })
        .insert(RigidBody::Fixed)
        // .insert(TesselatedCollider {
        //     texture: banners.clone(),
        //     ..default()
        // })
        .insert(Name::new("banners"));

}

fn pillar_position(
    player_query: Query<&GlobalTransform, With<Player>>,
    mut pillar_query: Query<&mut Transform, With<Pillar>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for mut pillar_transform in pillar_query.iter_mut() {
            if player_transform.translation().y >= (pillar_transform.translation.y-PILLAR_ADJUST) {
                pillar_transform.translation.z = PILLAR_Z_FRONT;
            } else {
                pillar_transform.translation.z = PILLAR_Z_BACK;
            }
        }
    }
}

fn spawn_pillars(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let column = asset_server.load("textures/temple/column.png");
    let column_hitbox = asset_server.load("textures/temple/colonne_hitbox.png");

    // let mut elements = Vec::new();
    // elements.push(t_banners);

    // All 6 PILLARS
    commands
        .spawn_bundle(SpriteBundle {
            texture: column.clone(),
            transform: Transform {
                translation: PILLAR_POSITION_1.into(),
                scale: TEMPLE_SCALE.into(),
                ..default()
            },
            ..SpriteBundle::default()
        })
        .insert(RigidBody::Fixed)
        .with_children(|parent| {
            parent
                .spawn()
                .insert(TesselatedCollider {
                    texture: column_hitbox.clone(),
                    tesselator_config: TesselatedColliderConfig {
                        vertice_separation: 0.,
                        ..default()
                    },
                    ..default()
                })
                .insert(Transform::from_xyz(0.0, PILLAR_HITBOX_Y_OFFSET, 0.0));
        })
        .insert(Pillar)
        .insert(Name::new("column 1"));

    commands
        .spawn_bundle(SpriteBundle {
            texture: column.clone(),
            transform: Transform {
                translation: PILLAR_POSITION_2.into(),
                scale: TEMPLE_SCALE.into(),
                ..default()
            },
            ..SpriteBundle::default()
        })
        .insert(RigidBody::Fixed)
        .with_children(|parent| {
            parent
                .spawn()
                .insert(TesselatedCollider {
                    texture: column_hitbox.clone(),
                    tesselator_config: TesselatedColliderConfig {
                        vertice_separation: 0.,
                        ..default()
                    },
                    ..default()
                })
                .insert(Transform::from_xyz(0.0, PILLAR_HITBOX_Y_OFFSET, 0.0));
        })
        .insert(Pillar)
        .insert(Name::new("column 2"));
        
    commands
        .spawn_bundle(SpriteBundle {
            texture: column.clone(),
            transform: Transform {
                translation: PILLAR_POSITION_3.into(),
                scale: TEMPLE_SCALE.into(),
                ..default()
            },
            ..SpriteBundle::default()
        })
        .insert(RigidBody::Fixed)
        .with_children(|parent| {
            parent
                .spawn()
                .insert(TesselatedCollider {
                    texture: column_hitbox.clone(),
                    tesselator_config: TesselatedColliderConfig {
                        vertice_separation: 0.,
                        ..default()
                    },
                    ..default()
                })
                .insert(Transform::from_xyz(0.0, PILLAR_HITBOX_Y_OFFSET, 0.0));
        })
        .insert(Pillar)
        .insert(Name::new("column 3"));

    
    commands
        .spawn_bundle(SpriteBundle {
            texture: column.clone(),
            transform: Transform {
                translation: PILLAR_POSITION_4.into(),
                scale: TEMPLE_SCALE.into(),
                ..default()
            },
            ..SpriteBundle::default()
        })
        .insert(RigidBody::Fixed)
        .with_children(|parent| {
            parent
                .spawn()
                .insert(TesselatedCollider {
                    texture: column_hitbox.clone(),
                    tesselator_config: TesselatedColliderConfig {
                        vertice_separation: 0.,
                        ..default()
                    },
                    ..default()
                })
                .insert(Transform::from_xyz(0.0, PILLAR_HITBOX_Y_OFFSET, 0.0));
        })
        .insert(Pillar)
        .insert(Name::new("column 4"));

    
    commands
        .spawn_bundle(SpriteBundle {
            texture: column.clone(),
            transform: Transform {
                translation: PILLAR_POSITION_5.into(),
                scale: TEMPLE_SCALE.into(),
                ..default()
            },
            ..SpriteBundle::default()
        })
        .insert(RigidBody::Fixed)
        .with_children(|parent| {
            parent
                .spawn()
                .insert(TesselatedCollider {
                    texture: column_hitbox.clone(),
                    tesselator_config: TesselatedColliderConfig {
                        vertice_separation: 0.,
                        ..default()
                    },
                    ..default()
                })
                .insert(Transform::from_xyz(0.0, PILLAR_HITBOX_Y_OFFSET, 0.0));
        })
        .insert(Pillar)
        .insert(Name::new("column 5"));

        
    commands
        .spawn_bundle(SpriteBundle {
            texture: column.clone(),
            transform: Transform {
                translation: PILLAR_POSITION_6.into(),
                scale: TEMPLE_SCALE.into(),
                ..default()
            },
            ..SpriteBundle::default()
        })
        .insert(RigidBody::Fixed)
        .with_children(|parent| {
            parent
                .spawn()
                .insert(TesselatedCollider {
                    texture: column_hitbox.clone(),
                    tesselator_config: TesselatedColliderConfig {
                        vertice_separation: 0.,
                        ..default()
                    },
                    ..default()
                })
                .insert(Transform::from_xyz(0.0, PILLAR_HITBOX_Y_OFFSET, 0.0));
        })
        .insert(Pillar)
        .insert(Name::new("column 6"));
}
