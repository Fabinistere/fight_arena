use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use log::info;

use crate::{
    npc::{
        movement::{PursuitBehavior, Target},
        NPC,
    },
    player::Player
};

#[derive(Component)]
pub struct DetectionSensor;

pub struct NPCDetectionEvent {
    pub started: bool,
}

// or Tracking
// #[derive(Component)]
// pub struct PursuitSensor;

pub fn threat_detection(
    mut commands: Commands,
    npc_query: Query<(Entity, &Name, &Children), (With<NPC>, Without<PursuitBehavior>)>,
    // Or potential enemy
    player_query: Query<(Entity, &Name, &Children), With<Player>>,
    rapier_context: Res<RapierContext>
) {

    let (player, player_name, player_colliders) = player_query.single();

    for (npc, npc_name, npc_colliders) in npc_query.iter() {

        // info!(target: "Entities checked", "{}", npc_name);
        for &npc_child in npc_colliders.iter() {
            for &player_child in player_colliders.iter() {
                /* Find the intersection pair, if it exists, between two colliders. */
                if rapier_context.intersection_pair(npc_child, player_child) == Some(true) {
                    info!(target: "Threat detected", "{:?} {} detected {:?} {}", npc, npc_name, player, player_name);
                    
                    commands
                        .entity(npc)
                        .insert(PursuitBehavior)
                        .insert(Target(Some(player)));

                    commands
                        .entity(npc)
                        .with_children(|parent| {
                            parent
                                .spawn()
                                .insert(Collider::ball(80.))
                                .insert(ActiveEvents::COLLISION_EVENTS)
                                .insert(Sensor)
                                .insert(Name::new("Pursuit Range"));
                        });

                    // give same orders to everyone in the group
                }
            }
        }
    }
}

// pub fn detection_trigger(
//     player_query: Query<&Transform, With<Player>>,
//     mut npc_query: Query<
//         (Entity,
//         &mut DetectionSensor,
//         &mut PursuitSensor,
//         &Name),
//         With<NPC>>,
//     mut detection_trigger_events: EventReader<NPCDetectionEvent>,
// ) {
//     for NPCDetectionEvent { started } in detection_trigger_events.iter() {
//         let transform = player_query.single();

//         if *started {

//         } else {
            
//         }
//     }
// }

// /* Change the collider sensor status inside of a system. */
// fn modify_collider_type(mut sensors: Query<&mut PursuitSensor, &Name>) {
//     for mut sensor in sensors.iter_mut() {
//         sensor.0 = true;
//     }
// }