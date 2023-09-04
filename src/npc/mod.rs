use std::collections::BTreeMap;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use yml_dialog::DialogNode;

use crate::{
    combat::{stats::*, GroupSize, Leader, Recruted, Team},
    constants::{
        character::{
            npc::{
                dialog::{FABIEN_DIALOG, OLF_DIALOG},
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
    ui::dialog_systems::DialogMap,
    FabienSheet,
};

pub mod aggression;
pub mod idle;
pub mod movement;

#[derive(Component, Reflect)]
pub struct NPC;

#[derive(Default)]
pub struct NPCPlugin;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
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
            .add_event::<aggression::StopChaseEvent>()
            .add_event::<aggression::DetectionModeEvent>()
            .add_event::<aggression::EngagePursuitEvent>()
            .add_systems(Startup, (spawn_characters, spawn_aggresives_characters))
            .add_systems(
                FixedUpdate,
                (
                    movement::just_walk.in_set(NPCSystems::Stroll),
                    idle::do_flexing
                        .in_set(NPCSystems::Idle)
                        .after(NPCSystems::Stroll),
                    movement::follow.in_set(NPCSystems::Follow),
                    aggression::add_detection_aura.before(NPCSystems::FindTargets),
                    aggression::threat_detection.in_set(NPCSystems::FindTargets),
                    aggression::add_pursuit_urge
                        .before(NPCSystems::Chase)
                        .after(NPCSystems::FindTargets),
                    movement::pursue
                        .in_set(NPCSystems::Chase)
                        .after(NPCSystems::FindTargets),
                    aggression::remove_pursuit_urge
                        .in_set(NPCSystems::StopChase)
                        .after(NPCSystems::Chase),
                    aggression::fair_play_wait.after(NPCSystems::StopChase),
                    aggression::add_detection_aura.after(NPCSystems::StopChase),
                ),
            );
    }
}

// Check in location/temple/mod.rs
// the npc_z_position

fn spawn_characters(mut commands: Commands, fabien: Res<FabienSheet>) {
    // ADMIRAL
    commands
        .spawn((
            SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(ADMIRAL_STARTING_ANIM),
                texture_atlas: fabien.0.clone(),
                transform: Transform {
                    translation: Vec3::new(-20., 35., NPC_Z_BACK),
                    scale: Vec3::splat(NPC_SCALE),
                    ..default()
                },
                ..default()
            },
            Name::new("NPC Admiral"),
            NPC,
            Team(TEAM_MC),
            Recruted,
            FollowupBehavior,
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            MovementBundle {
                speed: Speed::default(),
                velocity: Velocity {
                    linvel: Vect::ZERO,
                    angvel: 0.,
                },
            },
            CombatBundle {
                hp: HP::default(),
                mana: MANA::default(),
                initiative: Initiative::default(),
                attack: Attack::default(),
                attack_spe: AttackSpe::default(),
                defense: Defense::default(),
                defense_spe: DefenseSpe::default(),
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Collider::cuboid(CHAR_HITBOX_WIDTH, CHAR_HITBOX_HEIGHT),
                Transform::from_xyz(0., CHAR_HITBOX_Y_OFFSET, 0.),
                CharacterHitbox,
            ));
        });

    // HUGO
    commands
        .spawn((
            SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(HUGO_STARTING_ANIM),
                texture_atlas: fabien.0.clone(),
                transform: Transform {
                    translation: Vec3::new(-70., -55., NPC_Z_BACK),
                    scale: Vec3::splat(NPC_SCALE),
                    ..default()
                },
                ..default()
            },
            Name::new("NPC Hugo"),
            NPC,
            Team(TEAM_MC),
            Recruted,
            FollowupBehavior,
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            MovementBundle {
                speed: Speed::default(),
                velocity: Velocity {
                    linvel: Vect::ZERO,
                    angvel: 0.,
                },
            },
            CombatBundle {
                hp: HP::default(),
                mana: MANA::default(),
                initiative: Initiative::default(),
                attack: Attack::default(),
                attack_spe: AttackSpe::default(),
                defense: Defense::default(),
                defense_spe: DefenseSpe::default(),
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Collider::cuboid(CHAR_HITBOX_WIDTH, CHAR_HITBOX_HEIGHT),
                Transform::from_xyz(0., CHAR_HITBOX_Y_OFFSET, 0.),
                CharacterHitbox,
            ));
        });
}

fn spawn_aggresives_characters(
    mut commands: Commands,
    fabien: Res<FabienSheet>,
    mut dialogs: ResMut<DialogMap>,
) {
    /* -------------------------------------------------------------------------- */
    /*                                     OLF                                    */
    /* -------------------------------------------------------------------------- */
    let olf = commands
        .spawn((
            SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(OLF_STARTING_ANIM),
                texture_atlas: fabien.0.clone(),
                transform: Transform {
                    translation: Vec3::new(-20., 55., NPC_Z_BACK),
                    scale: Vec3::splat(NPC_SCALE),
                    ..default()
                },
                ..default()
            },
            Name::new("NPC Olf"),
            NPC,
            Leader,
            Team(TEAM_OLF),
            JustWalkBehavior {
                destination: give_a_direction(),
            },
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            MovementBundle {
                speed: Speed(NPC_SPEED_LEADER),
                velocity: Velocity {
                    linvel: Vect::ZERO,
                    angvel: 0.,
                },
            },
            CombatBundle {
                hp: HP::default(),
                mana: MANA::default(),
                initiative: Initiative::default(),
                attack: Attack::default(),
                attack_spe: AttackSpe::default(),
                defense: Defense::default(),
                defense_spe: DefenseSpe::default(),
            },
            // 5 Fabicurion are hidden within Olf's silhouette
            GroupSize(5),
            DetectionBehavior,
        ))
        .with_children(|parent| {
            parent.spawn((
                Collider::cuboid(CHAR_HITBOX_WIDTH, CHAR_HITBOX_HEIGHT),
                Transform::from_xyz(0., CHAR_HITBOX_Y_OFFSET, 0.),
                CharacterHitbox,
            ));

            parent.spawn((
                Collider::ball(40.),
                ActiveEvents::COLLISION_EVENTS,
                Sensor,
                DetectionSensor,
                Name::new("Detection Range"),
            ));
        })
        .id();

    let olf_deserialized_map: BTreeMap<usize, DialogNode> =
        serde_yaml::from_str(OLF_DIALOG).unwrap();
    dialogs.insert(
        olf,
        (
            *olf_deserialized_map.first_key_value().unwrap().0,
            olf_deserialized_map,
        ),
    );

    /* -------------------------------------------------------------------------- */
    /*                               Two FABICURION                               */
    /* -------------------------------------------------------------------------- */
    for i in 0..2 {
        let fabicurion = commands
            .spawn((
                SpriteSheetBundle {
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
                },
                Name::new(format!("NPC Fabicurion {}", i)),
                NPC,
                Leader,
                Team(TEAM_OLF),
                JustWalkBehavior {
                    destination: give_a_direction(),
                },
                RigidBody::Dynamic,
                LockedAxes::ROTATION_LOCKED,
                MovementBundle {
                    speed: Speed(NPC_SPEED),
                    velocity: Velocity {
                        linvel: Vect::ZERO,
                        angvel: 0.,
                    },
                },
                CombatBundle {
                    hp: HP::default(),
                    mana: MANA::default(),
                    initiative: Initiative::default(),
                    attack: Attack::default(),
                    attack_spe: AttackSpe::default(),
                    defense: Defense::default(),
                    defense_spe: DefenseSpe::default(),
                },
                // 2 Fabicurion are hidden behind the representant
                GroupSize(2),
                DetectionBehavior,
            ))
            .with_children(|parent| {
                parent.spawn((
                    Collider::cuboid(CHAR_HITBOX_WIDTH, CHAR_HITBOX_HEIGHT),
                    Transform::from_xyz(0., CHAR_HITBOX_Y_OFFSET, 0.),
                    CharacterHitbox,
                ));

                parent.spawn((
                    Collider::ball(40.),
                    ActiveEvents::COLLISION_EVENTS,
                    Sensor,
                    DetectionSensor,
                    Name::new("Detection Range"),
                ));
            })
            .id();

        let fabicurion_deserialized_map: BTreeMap<usize, DialogNode> =
            serde_yaml::from_str(FABIEN_DIALOG).unwrap();
        dialogs.insert(
            fabicurion,
            (
                *fabicurion_deserialized_map.first_key_value().unwrap().0,
                fabicurion_deserialized_map,
            ),
        );
    }
}
