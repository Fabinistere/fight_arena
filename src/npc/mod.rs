use bevy::time::FixedTimestep;
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::prelude::*;


use crate::{
    // collisions::{TesselatedCollider, TesselatedColliderConfig},
    combat::{
        Leader,
        Team,
        stats::*,
    },
    constants::FIXED_TIME_STEP,
    constants::{
        npc::{
            movement::NPC_SPEED_LEADER,
            NPC_SCALE,
            NPC_Z_BACK
        },
        combat::team::*
    },
    FabienSheet,
    movement::*,
    npc::{
        // idle::IdleBehavior,
        movement::FollowBehavior,
        movement::JustWalkBehavior,
        movement::give_a_direction
    }
};

pub mod movement;
pub mod idle;

#[derive(Component, Inspectable)]
pub struct NPC;

#[derive(Default)]
pub struct NPCPlugin;

#[derive(PartialEq, Clone, Hash, Debug, Eq, SystemLabel)]
pub enum NPCSystems {
    Stroll,
    Following,
    // FindLandmark,
    // FindTargets,
    // UpdateAggressionSource,
    // Talking,
    Idle,
    // Combat,
}

/**
 * NPC has hobbies
 *  - landwark
 *    - index in const, with free: bol
 *    - when talking to a npc in a landwark, include the other present
 *    -> rest
 *  - stroll
 *    - in a restricted zone -index in const-
 *    -> rest
 *  - rest
 *    -> stroll
 *    -> landwark
 *  - talking to MC
 *    - infite rest until the MC is leaving
 *    -> short rest
 *    or
 *    -> stroll
 *    -> landmark
 *    -> rest
 * 
 * Reflexion
 *  - should npc avoid hit other entity
 *  - turn false the free param from a landmark position taken by the MC
 */
impl Plugin  for NPCPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_character)
            .add_system_to_stage(
                CoreStage::Update,
                movement::just_walk
                    .with_run_criteria(FixedTimestep::step(FIXED_TIME_STEP as f64))
                    .label(NPCSystems::Stroll)
            )
            .add_system_to_stage(
                CoreStage::Update,
                movement::follow
                    .with_run_criteria(FixedTimestep::step(FIXED_TIME_STEP as f64))
                    .label(NPCSystems::Following)
            )
            .add_system_to_stage(
                CoreStage::Update,
                idle::do_flexing
                    .with_run_criteria(FixedTimestep::step(FIXED_TIME_STEP as f64))
                    .label(NPCSystems::Idle)
            );
    }
}

// Check in location/temple/mod.rs
// the npc_z_position

fn spawn_character(
    mut commands: Commands,
    fabien: Res<FabienSheet>,
    // asset_server: Res<AssetServer>,
) {

    let admiral = TextureAtlasSprite::new(0);

    let olf = TextureAtlasSprite::new(12);

    let hugo = TextureAtlasSprite::new(16);

    // let basic_hitbox = asset_server.load("textures/character/basic_hitbox.png");
    // let admiral_hitbox: Handle<Image> = asset_server.load("textures/character/admiral.png");
    // let olf_hitbox: Handle<Image> = asset_server.load("textures/character/Olf.png");

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: admiral,
            texture_atlas: fabien.0.clone(),
            transform: Transform {
                translation:  Vec3::new(-0.2, 0.35, NPC_Z_BACK),
                scale: Vec3::splat(NPC_SCALE),
                ..default()
            },
            ..default()
        }) 
        .insert(Name::new("NPC Admiral"))
        .insert(NPC)
        .insert(Team(TEAM_OLF))
        .insert(FollowBehavior)
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert_bundle(MovementBundle {
            speed: Speed::default(),
            velocity: Velocity {
                linvel: Vect::ZERO,
                angvel: 0.0,
            }
        })
        // .insert(GravityScale(0.01))
        // .insert(Sleeping::disabled())
        // .insert(Ccd::enabled())
        .insert_bundle(CombatBundle {
            hp: HP::default(),
            mana: MANA::default(),
            initiative: Initiative::default(),
            attack: Attack::default(),
            attack_spe: AttackSpe::default(),
            defense: Defense::default(),
            defense_spe: DefenseSpe::default()
        })
        // .insert(TesselatedCollider {
        //     texture: admiral_hitbox.clone(),
        //     tesselator_config: TesselatedColliderConfig::default(),
        // })
        ;

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: olf,
            texture_atlas: fabien.0.clone(),
            transform: Transform {
                translation:   Vec3::new(-0.2, 0.55, NPC_Z_BACK),
                scale: Vec3::splat(NPC_SCALE),
                ..default()
            },
            ..default()
        }) 
        .insert(Name::new("NPC Olf"))
        .insert(NPC)
        .insert(Leader)
        .insert(Team(TEAM_OLF))
        // .insert(IdleBehavior)
        .insert(JustWalkBehavior {
            destination: give_a_direction()
        })
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert_bundle(MovementBundle {
            speed: Speed(NPC_SPEED_LEADER),
            velocity: Velocity {
                linvel: Vect::ZERO,
                angvel: 0.0,
            }
        })
        // .insert(GravityScale(0.01))
        // .insert(Sleeping::disabled())
        // .insert(Ccd::enabled())
        .insert_bundle(CombatBundle {
            hp: HP::default(),
            mana: MANA::default(),
            initiative: Initiative::default(),
            attack: Attack::default(),
            attack_spe: AttackSpe::default(),
            defense: Defense::default(),
            defense_spe: DefenseSpe::default()
        })
        // .insert(TesselatedCollider {
        //     texture: olf_hitbox.clone(),
        //     tesselator_config: TesselatedColliderConfig::default()
        // })
        ;

        commands
            .spawn_bundle(SpriteSheetBundle {
                sprite: hugo,
                texture_atlas: fabien.0.clone(),
                transform: Transform {
                    translation:  Vec3::new(-0.7, -0.55, NPC_Z_BACK),
                    scale: Vec3::splat(NPC_SCALE),
                    ..default()
                },
                ..default()
            }) 
            .insert(Name::new("NPC Hugo"))
            .insert(NPC)
            .insert(Team(TEAM_OLF))
            .insert(FollowBehavior)
            .insert(RigidBody::Dynamic)
            .insert(LockedAxes::ROTATION_LOCKED)
            .insert_bundle(MovementBundle {
                speed: Speed::default(),
                velocity: Velocity {
                    linvel: Vect::ZERO,
                    angvel: 0.0,
                }
            })
            .insert_bundle(CombatBundle {
                hp: HP::default(),
                mana: MANA::default(),
                initiative: Initiative::default(),
                attack: Attack::default(),
                attack_spe: AttackSpe::default(),
                defense: Defense::default(),
                defense_spe: DefenseSpe::default()
            })
            ;
}
