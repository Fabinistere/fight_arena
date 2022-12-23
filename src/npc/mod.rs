use bevy::prelude::*;
use bevy::time::FixedTimestep;
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::prelude::*;

use crate::{
    combat::{stats::*, GroupSize, Leader, Recruted, Team},
    constants::FIXED_TIME_STEP,
    constants::{
        character::{
            npc::{
                dialog::{OLF_DIALOG, FABIEN_DIALOG},
                movement::{NPC_SPEED, NPC_SPEED_LEADER},
                *,
            },
            CHAR_HITBOX_HEIGHT, CHAR_HITBOX_WIDTH, CHAR_HITBOX_Y_OFFSET,
        },
        combat::team::*,
    },
    movement::*,
    npc::{
        aggression::DetectionSensor,
        // idle::IdleBehavior,
        movement::{give_a_direction, DetectionBehavior, FollowupBehavior, JustWalkBehavior},
    },
    ui::dialog_system::Dialog,
    FabienSheet,
};

pub mod aggression;
pub mod idle;
pub mod movement;

#[derive(Component, Inspectable)]
pub struct NPC;

#[derive(Default)]
pub struct NPCPlugin;

#[derive(PartialEq, Clone, Hash, Debug, Eq, SystemLabel)]
pub enum NPCSystems {
    Stroll,
    Follow,
    // FindLandmark,
    FindTargets,
    Chase,
    StopChase,
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
impl Plugin for NPCPlugin {
    fn build(&self, app: &mut App) {
        app
            // when an enemy npc catch the player or an ally attached to the group
            // initialize a Combat
            // Combat mean A lock dialogue : Talk or Fight
            .add_event::<aggression::CombatEvent>()
            .add_event::<aggression::CombatExitEvent>()
            .add_event::<aggression::StopChaseEvent>()
            .add_event::<aggression::DetectionModeEvent>()
            .add_event::<aggression::EngagePursuitEvent>()
            .add_startup_system(spawn_characters)
            .add_startup_system(spawn_aggresives_characters)
            .add_system_set_to_stage(
                CoreStage::Update,
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(FIXED_TIME_STEP as f64))
                    .with_system(movement::just_walk.label(NPCSystems::Stroll))
                    .with_system(movement::follow.label(NPCSystems::Follow))
                    .with_system(
                        idle::do_flexing
                            .label(NPCSystems::Idle)
                            .after(NPCSystems::Stroll),
                    ),
            )
            // .add_system(aggression::add_pursuit_urge)
            // .add_system(aggression::remove_pursuit_urge)
            .add_system_to_stage(
                CoreStage::Update,
                aggression::add_detection_aura
                    .with_run_criteria(FixedTimestep::step(FIXED_TIME_STEP as f64))
                    .before(NPCSystems::FindTargets),
            )
            .add_system_to_stage(
                CoreStage::Update,
                aggression::threat_detection
                    .with_run_criteria(FixedTimestep::step(FIXED_TIME_STEP as f64))
                    .label(NPCSystems::FindTargets),
            )
            .add_system_to_stage(
                CoreStage::Update,
                aggression::add_pursuit_urge
                    .with_run_criteria(FixedTimestep::step(FIXED_TIME_STEP as f64))
                    .before(NPCSystems::Chase)
                    .after(NPCSystems::FindTargets),
            )
            .add_system_to_stage(
                CoreStage::Update,
                movement::pursue
                    .with_run_criteria(FixedTimestep::step(FIXED_TIME_STEP as f64))
                    .label(NPCSystems::Chase)
                    .after(NPCSystems::FindTargets),
            )
            .add_system_to_stage(
                CoreStage::Update,
                aggression::remove_pursuit_urge
                    .with_run_criteria(FixedTimestep::step(FIXED_TIME_STEP as f64))
                    .label(NPCSystems::StopChase)
                    .after(NPCSystems::Chase),
            )
            .add_system_to_stage(
                CoreStage::Update,
                aggression::fair_play_wait
                    .with_run_criteria(FixedTimestep::step(FIXED_TIME_STEP as f64))
                    .after(NPCSystems::StopChase),
            )
            .add_system(
                aggression::add_detection_aura
                    .with_run_criteria(FixedTimestep::step(FIXED_TIME_STEP as f64))
                    .after(NPCSystems::StopChase),
            );
    }
}

// Check in location/temple/mod.rs
// the npc_z_position

fn spawn_characters(mut commands: Commands, fabien: Res<FabienSheet>) {
    // ADMIRAL
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(ADMIRAL_STARTING_ANIM),
            texture_atlas: fabien.0.clone(),
            transform: Transform {
                translation: Vec3::new(-20., 35., NPC_Z_BACK),
                scale: Vec3::splat(NPC_SCALE),
                ..default()
            },
            ..default()
        })
        .insert(Name::new("NPC Admiral"))
        .insert(NPC)
        .insert(Team(TEAM_MC))
        .insert(Recruted)
        .insert(FollowupBehavior)
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert_bundle(MovementBundle {
            speed: Speed::default(),
            velocity: Velocity {
                linvel: Vect::ZERO,
                angvel: 0.0,
            },
        })
        .insert_bundle(CombatBundle {
            hp: HP::default(),
            mana: MANA::default(),
            initiative: Initiative::default(),
            attack: Attack::default(),
            attack_spe: AttackSpe::default(),
            defense: Defense::default(),
            defense_spe: DefenseSpe::default(),
        })
        .with_children(|parent| {
            parent
                .spawn()
                .insert(Collider::cuboid(CHAR_HITBOX_WIDTH, CHAR_HITBOX_HEIGHT))
                .insert(Transform::from_xyz(0.0, CHAR_HITBOX_Y_OFFSET, 0.0))
                .insert(CharacterHitbox);
        });

    // HUGO
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(HUGO_STARTING_ANIM),
            texture_atlas: fabien.0.clone(),
            transform: Transform {
                translation: Vec3::new(-70., -55., NPC_Z_BACK),
                scale: Vec3::splat(NPC_SCALE),
                ..default()
            },
            ..default()
        })
        .insert(Name::new("NPC Hugo"))
        .insert(NPC)
        .insert(Team(TEAM_MC))
        .insert(Recruted)
        .insert(FollowupBehavior)
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert_bundle(MovementBundle {
            speed: Speed::default(),
            velocity: Velocity {
                linvel: Vect::ZERO,
                angvel: 0.0,
            },
        })
        .insert_bundle(CombatBundle {
            hp: HP::default(),
            mana: MANA::default(),
            initiative: Initiative::default(),
            attack: Attack::default(),
            attack_spe: AttackSpe::default(),
            defense: Defense::default(),
            defense_spe: DefenseSpe::default(),
        })
        .with_children(|parent| {
            parent
                .spawn()
                .insert(Collider::cuboid(CHAR_HITBOX_WIDTH, CHAR_HITBOX_HEIGHT))
                .insert(Transform::from_xyz(0.0, CHAR_HITBOX_Y_OFFSET, 0.0))
                .insert(CharacterHitbox);
        });
}

fn spawn_aggresives_characters(mut commands: Commands, fabien: Res<FabienSheet>) {
    // let olf_dialog_tree = init_tree_flat(String::from(OLF_DIALOG));

    // OLF
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(OLF_STARTING_ANIM),
            texture_atlas: fabien.0.clone(),
            transform: Transform {
                translation: Vec3::new(-20., 55., NPC_Z_BACK),
                scale: Vec3::splat(NPC_SCALE),
                ..default()
            },
            ..default()
        })
        .insert(Name::new("NPC Olf"))
        .insert(NPC)
        .insert(Leader)
        .insert(Team(TEAM_OLF))
        .insert(JustWalkBehavior {
            destination: give_a_direction(),
        })
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert_bundle(MovementBundle {
            speed: Speed(NPC_SPEED_LEADER),
            velocity: Velocity {
                linvel: Vect::ZERO,
                angvel: 0.0,
            },
        })
        .insert_bundle(CombatBundle {
            hp: HP::default(),
            mana: MANA::default(),
            initiative: Initiative::default(),
            attack: Attack::default(),
            attack_spe: AttackSpe::default(),
            defense: Defense::default(),
            defense_spe: DefenseSpe::default(),
        })
        .insert(Dialog{ current_node: Some(String::from(OLF_DIALOG))})
        // 5 Fabicurion are hidden within Olf's silhouette
        .insert(GroupSize(5))
        .insert(DetectionBehavior)
        .with_children(|parent| {
            parent
                .spawn()
                .insert(Collider::cuboid(CHAR_HITBOX_WIDTH, CHAR_HITBOX_HEIGHT))
                .insert(Transform::from_xyz(0.0, CHAR_HITBOX_Y_OFFSET, 0.0))
                .insert(CharacterHitbox);

            parent
                .spawn()
                .insert(Collider::ball(40.))
                .insert(ActiveEvents::COLLISION_EVENTS)
                .insert(Sensor)
                .insert(DetectionSensor)
                .insert(Name::new("Detection Range"));
        });

    // Two FABICURION
    for i in 0..2 {
        let name = "NPC Fabicurion nmb".replace("nmb", &i.to_string());

        commands
            .spawn_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(FABICURION_STARTING_ANIM),
                texture_atlas: fabien.0.clone(),
                transform: Transform {
                    translation: Vec3::new(
                        -20. + (i * 10) as f32,
                        55. + (i * 10) as f32,
                        NPC_Z_BACK,
                    ),
                    scale: Vec3::splat(NPC_SCALE),
                    ..default()
                },
                ..default()
            })
            .insert(Name::new(name))
            .insert(NPC)
            .insert(Leader)
            .insert(Team(TEAM_OLF))
            .insert(JustWalkBehavior {
                destination: give_a_direction(),
            })
            .insert(RigidBody::Dynamic)
            .insert(LockedAxes::ROTATION_LOCKED)
            .insert_bundle(MovementBundle {
                speed: Speed(NPC_SPEED),
                velocity: Velocity {
                    linvel: Vect::ZERO,
                    angvel: 0.0,
                },
            })
            .insert_bundle(CombatBundle {
                hp: HP::default(),
                mana: MANA::default(),
                initiative: Initiative::default(),
                attack: Attack::default(),
                attack_spe: AttackSpe::default(),
                defense: Defense::default(),
                defense_spe: DefenseSpe::default(),
            })
            // 2 Fabicurion are hidden behind the representant
            .insert(GroupSize(2))
            .insert(DetectionBehavior)
            .insert(Dialog{ current_node: Some(String::from(FABIEN_DIALOG))})
            .with_children(|parent| {
                parent
                    .spawn()
                    .insert(Collider::cuboid(CHAR_HITBOX_WIDTH, CHAR_HITBOX_HEIGHT))
                    .insert(Transform::from_xyz(0.0, CHAR_HITBOX_Y_OFFSET, 0.0))
                    .insert(CharacterHitbox);

                parent
                    .spawn()
                    .insert(Collider::ball(40.))
                    .insert(ActiveEvents::COLLISION_EVENTS)
                    .insert(Sensor)
                    .insert(DetectionSensor)
                    .insert(Name::new("Detection Range"));
            });
    }
}
