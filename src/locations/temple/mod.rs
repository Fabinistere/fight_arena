use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::Location;
use crate::{
    collisions::{TesselatedCollider, TesselatedColliderConfig},
    constants::{
        character::npc::{NPC_Z_BACK, NPC_Z_FRONT},
        locations::temple::*,
    },
    npc::NPC,
    player::Player,
};

pub struct TemplePlugin;

impl Plugin for TemplePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<PlayerLocation>()
            .add_event::<SpawnPillarEvent>()
            .add_systems((setup_temple, spawn_pillars).in_schedule(OnEnter(Location::Temple)))
            .add_systems(
                (throne_position, pillar_position, npc_z_position)
                    // CoreSet::PostUpdate
                    .in_set(OnUpdate(Location::Temple)),
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

/// Happens in
///   - location::temple::mod
///     - setup_temple
/// Read in
///   - location::temple::mod
///     - spawn_pillars
struct SpawnPillarEvent;

/// REFACTOR: Use Location only ?
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum PlayerLocation {
    #[default]
    Temple,
}

/// XXX: doesn't work well
fn npc_z_position(
    mut npc_query: Query<&mut Transform, With<NPC>>,
    pillar_query: Query<&GlobalTransform, With<Pillar>>,
) {
    // TODO: prevent no transform in npc
    for mut npc_transform in npc_query.iter_mut() {
        for pillar_transform in pillar_query.iter() {
            // put the npc in front cause above the last pillar
            // the pb was: always below one pillar (the closest to the stage)
            // It only works when the npc was below the last pillar
            // this methods doesn't work cause we can be ABOVE and BELOW two diff pillars
            // between two line (in a single column)
            if npc_transform.translation.y <= pillar_transform.translation().y + 0.07
                && npc_transform.translation.y >= pillar_transform.translation().y - 0.07
            {
                if npc_transform.translation.y >= (pillar_transform.translation().y - PILLAR_ADJUST)
                {
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
fn setup_temple(mut commands: Commands, asset_server: Res<AssetServer>) {
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
    let huge_throne_hitbox: Handle<Image> =
        asset_server.load("textures/temple/temple_huge_throne_hitbox.png");

    let corridors = asset_server.load("textures/temple/corridors.png");

    commands.spawn((
        SpriteBundle {
            texture: corridors.clone(),
            transform: Transform {
                translation: Vec3::new(0., 0., 9.),
                scale: TEMPLE_SCALE.into(),
                ..default()
            },
            ..SpriteBundle::default()
        },
        RigidBody::Fixed,
        Name::new("corridors"),
    ));

    commands
        .spawn((
            SpriteBundle {
                texture: wall.clone(),
                transform: Transform {
                    translation: Vec3::new(0., 0., TEMPLE_Z),
                    scale: TEMPLE_SCALE.into(),
                    ..default()
                },
                ..SpriteBundle::default()
            },
            RigidBody::Fixed,
            Name::new("wall")
        ))
        // .insert(TesselatedCollider {
        //     texture: wall.clone(),
        //     ..default()
        // })
        ;

    commands.spawn((
        SpriteBundle {
            texture: floor.clone(),
            transform: Transform {
                translation: TEMPLE_POSITION.into(),
                scale: TEMPLE_SCALE.into(),
                ..default()
            },
            ..SpriteBundle::default()
        },
        RigidBody::Fixed,
        Name::new("floor"),
    ));

    commands
        .spawn((
            SpriteBundle {
                texture: huge_throne.clone(),
                transform: Transform {
                    translation: THRONE_POSITION.into(),
                    scale: TEMPLE_SCALE.into(),
                    ..default()
                },
                ..SpriteBundle::default()
            },
            RigidBody::Fixed,
            Throne,
            Name::new("throne"),
        ))
        .with_children(|parent| {
            parent.spawn((
                TesselatedCollider {
                    texture: huge_throne_hitbox.clone(),
                    tesselator_config: TesselatedColliderConfig {
                        vertice_separation: 0.,
                        extrusion: 0.1,
                        vertice_radius: 0.4,
                    },
                },
                Transform::from_xyz(0.0, 0., 0.0),
                Name::new("Throne Hitbox"),
            ));
        });

    commands
        .spawn((
            SpriteBundle {
                texture: banners.clone(),
                transform: Transform {
                    translation: BANNERS_POSITION.into(),
                    scale: TEMPLE_SCALE.into(),
                    ..default()
                },
                ..SpriteBundle::default()
            },
            RigidBody::Fixed,
            Name::new("banners")
        ))
        // .insert(TesselatedCollider {
        //     texture: banners.clone(),
        //     ..default()
        // })
        ;
}

fn pillar_position(
    player_query: Query<&GlobalTransform, With<Player>>,
    mut pillar_query: Query<&mut Transform, With<Pillar>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for mut pillar_transform in pillar_query.iter_mut() {
            if player_transform.translation().y >= (pillar_transform.translation.y - PILLAR_ADJUST)
            {
                pillar_transform.translation.z = PILLAR_Z_FRONT;
            } else {
                pillar_transform.translation.z = PILLAR_Z_BACK;
            }
        }
    }
}

fn spawn_pillars(mut commands: Commands, asset_server: Res<AssetServer>) {
    let column = asset_server.load("textures/temple/column.png");
    let column_hitbox = asset_server.load("textures/temple/colonne_hitbox.png");

    // let mut elements = Vec::new();
    // elements.push(t_banners);

    // TODO: CHECK https://bevy-cheatbook.github.io/features/parent-child.html

    // REFACTOR: iterate into all pillar const
    let pillars_position = vec![
        PILLAR_POSITION_1,
        PILLAR_POSITION_2,
        PILLAR_POSITION_3,
        PILLAR_POSITION_4,
        PILLAR_POSITION_5,
        PILLAR_POSITION_6,
    ];

    // All 6 PILLARS
    for count in 1..7 {
        let name = format!("column {}", count);
        commands
            .spawn((
                SpriteBundle {
                    texture: column.clone(),
                    transform: Transform {
                        // a vector of position or anything else
                        translation: pillars_position[count - 1].into(),
                        scale: TEMPLE_SCALE.into(),
                        ..default()
                    },
                    ..SpriteBundle::default()
                },
                RigidBody::Fixed,
                Pillar,
                Name::new(name),
            ))
            .with_children(|parent| {
                parent.spawn((
                    TesselatedCollider {
                        texture: column_hitbox.clone(),
                        tesselator_config: TesselatedColliderConfig {
                            vertice_separation: 0.,
                            ..default()
                        },
                        ..default()
                    },
                    Transform::from_xyz(0.0, PILLAR_HITBOX_Y_OFFSET, 0.0),
                ));
            });
    }
}
