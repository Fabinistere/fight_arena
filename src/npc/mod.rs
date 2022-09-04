use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::prelude::*;
use bevy_retrograde::prelude::{TesselatedCollider, TesselatedColliderConfig};

use crate::{
    constants::FIXED_TIME_STEP,
    constants::npc::movement::NPC_SPEED,
    FabienSheet,
    movement::*,
    npc::{
        // idle::IdleBehavior,
        movement::FollowBehavior,
        // movement::JustWalkBehavior,
        // movement::give_a_direction
    },
    spawn_fabien_sprite
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
            // .add_startup_system(show_ieud_grid)
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
            // .add_system_to_stage(
            //     CoreStage::Update,
            //     movement::find_landmark
            //         .with_run_criteria(FixedTimestep::step(FIXED_TIME_STEP as f64))
            //         .label(NPCSystems::FindLandmark)
            // )
            // .add_system_to_stage(
            //     CoreStage::Update,
            //     idle::talking
            //         .with_run_criteria(FixedTimestep::step(FIXED_TIME_STEP as f64))
            //         .label(NPCSystems::Talking)
            // )
            .add_system_to_stage(
                CoreStage::Update,
                idle::do_flexing
                    .with_run_criteria(FixedTimestep::step(FIXED_TIME_STEP as f64))
                    .label(NPCSystems::Idle)
            );
    }
}

fn spawn_character(
    mut commands: Commands,
    fabien: Res<FabienSheet>,
    asset_server: Res<AssetServer>
) {
    let position = Vec3::new(-0.2, 0.35, 5.);

    let admiral = spawn_fabien_sprite(
        &mut commands,
        &fabien,
        0,
        Color::rgb(0.9,0.9,0.9),
        position,
        Vec3::new(2.0,2.0,0.0)
    );

    let olf = spawn_fabien_sprite(
        &mut commands,
        &fabien,
        12,
        Color::rgb(0.9,0.9,0.9),
        Vec3::new(-0.2, 0.55, 5.),
        Vec3::new(2.0,2.0,0.0)
    );

    // let basic_hitbox = asset_server.load("textures/character/basic_hitbox.png");
    let admiral_hitbox = asset_server.load("textures/character/admiral.png");
    let olf_hitbox = asset_server.load("textures/character/Olf.png");

    commands
        .entity(admiral)
        .insert(Name::new("NPC Admiral"))
        .insert(NPC)
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert_bundle(MovementBundle {
            speed: Speed(NPC_SPEED),
            velocity: Velocity {
                linvel: Vec2::default(),
                angvel: 0.0,
            }
        })
        .insert(TesselatedCollider {
            texture: admiral_hitbox.clone(),
            tesselator_config: TesselatedColliderConfig {
                // We want the collision shape for the player to be highly accurate
                vertice_separation: 0.,
                ..default()
            },
            ..default()
        })
        .insert(FollowBehavior);

    commands
        .entity(olf)
        .insert(Name::new("NPC Olf"))
        .insert(NPC)
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert_bundle(MovementBundle {
            speed: Speed(NPC_SPEED),
            velocity: Velocity {
                linvel: Vec2::default(),
                angvel: 0.0,
            }
        })
        .insert(TesselatedCollider {
            texture: olf_hitbox.clone(),
            tesselator_config: TesselatedColliderConfig {
                // We want the collision shape for the player to be highly accurate
                vertice_separation: 0.,
                ..default()
            },
            ..default()
        })
        .insert(FollowBehavior);
}

fn _show_ieud_grid(
    mut commands: Commands,
    fabien: Res<FabienSheet>
) {
    // TODO proper GRID

    let mut marks = Vec::new();

    for i in -10..10 {
        for j in -10..10 {
            let mark = spawn_fabien_sprite(
                &mut commands,
                &fabien,
                16,
                Color::rgb(0.9,0.9,0.9),
                Vec3::new(i as f32*0.1, j as f32*0.1, 4.0),
                Vec3::new(1.0,1.0,0.0)
            );
            let _name = 
                "Mark {a}.{b}".replace("{a}", &(i+10).to_string())
                              .replace("{b}", &(j+10).to_string());
            
            marks.push(mark);

            // commands
            //     .entity(mark)
            //     .insert(Name::new(name));
        }
    }

    commands
        .spawn()
        .insert(Name::new("Marks"))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .push_children(&marks);
}