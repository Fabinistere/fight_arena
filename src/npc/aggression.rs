use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
// use log::info;

use crate::{
    collisions::CollisionEventExt,
    combat::{ FairPlayTimer, Team },
    constants::{character::npc::movement::EVASION_TIMER, combat::team::TEAM_MC},
    npc::{
        movement::{
            PursuitBehavior,
            Target,
            DetectionBehavior
        },
        NPC,
    },
    movement::CharacterHitbox,
};

#[derive(Component)]
pub struct DetectionSensor;

#[derive(Component)]
pub struct PursuitSensor;

// pub struct NPCDetectionEvent {
//     pub npc_entity: Entity,
//     pub deteted_entity: Entity,
// }

/// Happens when:
///   - npc::movement::pursue
///     - target is reach
/// Read in
///   - ui::dialog_box::create_dialog_box_on_combat_event
///     - open combat ui
///   - *future*
///     - freeze all entities involved in the starting combat
pub struct CombatEvent;

/// Happens when:
///   - combat::mod
///     - combat ended
/// Read in
///   - ui::dialog_box::create_dialog_box_on_combat_event
///     - close the ui
pub struct CombatExitEvent;

/// Happens when:
///   - npc::movement::pursue
///     - target is not found/exist
///     - target is reach
/// Read in npc::aggression::remove_pursuit_urge
pub struct StopChaseEvent {
    pub npc_entity: Entity
}

/// Happens when:
///   - npc::mod
///     - creating npc
/// Read in
///   - npc::aggression::add_detection_aura
///     - creates THE DetectionSensor in the entity
pub struct DetectionModeEvent {
    pub entity: Entity
}

/// Happens when:
///   - npc::aggression::threat_detection
///     - An npc detected a enemy
/// Read in
///   - npc::aggression::add_pursuit_urge
///     - remove DetectionBehavior from the entity
///     - insert PursuitBehavior into the entity
///     - insert the Target into the entity
pub struct EngagePursuitEvent {
    npc_entity: Entity,
    detection_sensor_entity: Entity,
    target_entity: Entity
}

/// targeting after the detection event
/// Even if the FairPlayTimer ended:
///     As the entity doesn't stop the collision to restart it,
///     (quit n enter the detection circle)
///     with themself and the DetectionSensor,
///     the npc won't start pursue/chase
///     (wait for you to hide)
pub fn threat_detection(
    mut ev_engage_pursuit: EventWriter<EngagePursuitEvent>,

    rapier_context: Res<RapierContext>,

    mut collision_events: EventReader<CollisionEvent>,
    collider_sensor_query: Query<(Entity, &Parent), (With<Collider>, With<Sensor>, With<DetectionSensor>)>,
    collider_query: Query<(Entity, &Parent), (With<Collider>, With<CharacterHitbox>)>,
    
    target_query: Query<(Entity, &Team, &Name)>,
    npc_query: Query<(Entity, &Team, &Name), (With<NPC>, With<DetectionBehavior>, Without<PursuitBehavior>, Without<FairPlayTimer>)>
) {

    for collision_event in collision_events.iter() {
        let entity_1 = collision_event.entities().0;
        let entity_2 = collision_event.entities().1;
        
        // one of theses two colliders is a sensor
        if rapier_context.intersection_pair(entity_1, entity_2) == Some(true) {

            // DEBUG: info!(target: "Collision Event with a sensor involved", "{:?} and {:?}", entity_1, entity_2);

            match (
                collider_sensor_query.get(entity_1),
                collider_sensor_query.get(entity_2),
                collider_query.get(entity_1),
                collider_query.get(entity_2)
            ) {
                // only one of them is a ColliderSensor: sensor_potential_npc
                // and the other one is a hitbox_potential_threat
                (Ok(sensor_potential_npc), Err(_e1), Err(_e2), Ok(hitbox_potential_threat))
                | (Err(_e1), Ok(sensor_potential_npc), Ok(hitbox_potential_threat), Err(_e2)) => {

                    // DEBUG: info!(target: "Collision with a sensor and a hitbox", "{:?} and {:?}", sensor_potential_npc, hitbox_potential_threat);

                    // [sensor_potential_npc, hitbox_potential_threat].1 returns the Parent Entity

                    // from the collider get their parent
                    match (
                        npc_query.get(sensor_potential_npc.1.get()),
                        target_query.get(hitbox_potential_threat.1.get())
                    ) {
                        (Ok(npc), Ok(target)) => {

                            // DEBUG: info!(target: "Collision with a npc and a character", "{:?} and {:?}", npc.0, target.0);

                            // [npc, target].0: Entity
                            // [npc, target].1: &Team
                            // [npc, target].2: &Name
                            // add the potential_threat as a target if not in the same team
                            if target.1 != npc.1 {

                                info!("{} detected {}: chase initialized", npc.2, target.2);

                                // turn off npc detection sensor
                                // turn on npc pursuit sensor
                                // insert the new target into the npc
                                ev_engage_pursuit.send(EngagePursuitEvent {
                                    npc_entity: npc.0,
                                    detection_sensor_entity: sensor_potential_npc.0,
                                    target_entity: target.0
                                });
                                

                            }
                            else { 
                                info!("{} detected {}: same team", npc.2, target.2);
                                continue
                            }
                            
                        }

                        // not our manners (not a npc OR not a potential target)
                        (Err(e), _) => warn!(target: "Not an NPC", "err: {:?}", e),
                        (_, Err(e)) => warn!(target: "Not an Targeable Entity", "err: {:?}", e),

                        // _ => continue
                    }

                }
                // two are sensors
                // two are errors
                _ => continue,
            }
        }
        
        
        // DEBUG: println!("Received collision event: {:?}", collision_event);
        
        
    }
}

/// Decrement the fair play Timer
/// while doing other things (don't **exclude** entity With<FairPlayTimer>)
/// remove the FairPlayTimer if the entity is in the player's team
pub fn fair_play_wait(
    mut commands: Commands,

    time: Res<Time>,
    mut npc_query: Query<
        (Entity, &mut FairPlayTimer, &mut Velocity, &Team, &Name), 
        (
            With<NPC>,
        )
    >
) {
    for (npc, mut fair_play_timer, mut _rb_vel, team, name) in npc_query.iter_mut() {

        fair_play_timer.timer.tick(time.delta());

        // not required to control velocity because it is managed elsewhere

        // TODO query player to get his TEAM (it's the player who switch team not all npc)
        if fair_play_timer.timer.finished() || team.0 == TEAM_MC {

            info!("{:?}, {} can now aggro", npc, name);

            commands.entity(npc)
                    .remove::<FairPlayTimer>();
        }          

    }
}

/// - turn off npc detection sensor
/// - turn on npc pursuit sensor
/// - insert the new target into the npc
/// match the ev's args in a query ? => security
pub fn add_pursuit_urge(
    mut commands: Commands,
    mut ev_engage_pursuit: EventReader<EngagePursuitEvent>,
    npc_query: Query<
        Entity,
        (
            With<NPC>,
            With<DetectionBehavior>,
            Without<PursuitBehavior>,
            Without<FairPlayTimer>
        )>,
) {

    // let pursuit_sensor =
    //     commands
    //         .spawn()
    //         .insert(Collider::ball(80.))
    //         .insert(ActiveEvents::COLLISION_EVENTS)
    //         .insert(Sensor)
    //         .insert(PursuitSensor)
    //         .insert(Name::new("Pursuit Range"))
    //         .id();

    for ev in ev_engage_pursuit.iter() {

        info!("help");
        
        match npc_query.get(ev.npc_entity) {

            Ok(npc) => {
                info!("add pursuit urge with an npc");

                // remove DetectionSensor
                commands
                    .entity(ev.detection_sensor_entity)
                    .despawn();

                // remove DetectionBehavior
                commands
                    .entity(npc)
                    .remove::<DetectionBehavior>();
        
                // turn on npc pursuit sensor
                commands
                    .entity(npc)
                    .insert(PursuitBehavior)
                    .with_children(|parent| {
                        parent
                            .spawn()
                            .insert(Collider::ball(80.))
                            .insert(ActiveEvents::COLLISION_EVENTS)
                            .insert(Sensor)
                            .insert(PursuitSensor)
                            .insert(Name::new("Pursuit Range"));
                    });                       
        
                // insert the new target into the npc
                commands
                    .entity(npc)
                    .insert(Target(Some(ev.target_entity)));
            }

            _ => continue
        }
        
    }
}

/// remove target
/// remove PursuitBehavior
pub fn remove_pursuit_urge (
    mut commands: Commands,
    mut ev_stop_chase: EventReader<StopChaseEvent>,
    npc_query: Query<(Entity, &Children), (With<NPC>, With<PursuitBehavior>)>,
    pursuit_sensor_query: Query<Entity, (With<Collider>, With<Sensor>, With<PursuitSensor>)>,

    mut ev_detection_mode: EventWriter<DetectionModeEvent>
) {
    for ev in ev_stop_chase.iter()
    {

        // remove PursuitSensor Collider (or turn it false, one day)
        match npc_query.get(ev.npc_entity) {

            Ok(npc) => {
                info!("remove pursuit urge with an npc");

                commands
                    .entity(npc.0)
                    .remove::<PursuitBehavior>();

                commands
                    .entity(npc.0)
                    .remove::<Target>();

                commands
                    .entity(npc.0)
                    .insert(FairPlayTimer {
                        // create the non-repeating rest timer
                        timer: Timer::new(Duration::from_secs(EVASION_TIMER), false),
                    });

                // insert DetectionSensor into the Entity npc.0
                // insert DetectionBehavior
                ev_detection_mode.send(DetectionModeEvent {
                    entity: npc.0
                });

                // browse all colliders contained in within the npc
                for collider in npc.1.iter() {

                    // for all colliders matching with our query pursuit_sensor_query
                    // despawn it
                    match pursuit_sensor_query.get(*collider) {
                        // returned pursuit_sensor: Entity
                        Ok(pursuit_sensor) => {
                            commands
                                .entity(pursuit_sensor)
                                .despawn();
                        }

                        _ => continue
                    }

                }

            }

            _ => continue
        }
    }

    // send even to:
    // back to normal behavior
    // with prioritize behavior
}

/// Insert DetectionSensor
/// Insert DetectionBehavior
pub fn add_detection_aura(
    mut commands: Commands,
    mut ev_detection_mode: EventReader<DetectionModeEvent>,

    npc_query: Query<Entity, (With<NPC>, Without<DetectionBehavior>)>,
) {
    // let detection_sensor =
    //     commands
    //         .spawn()
    //         .insert(Collider::ball(40.))
    //         .insert(ActiveEvents::COLLISION_EVENTS)
    //         .insert(Sensor)
    //         .insert(DetectionSensor)
    //         .insert(Name::new("Detection Range"))
    //         .id();

    // info!("enter add detection aura");
    
    for ev in ev_detection_mode.iter() {

        info!("detection mode ev");

        // verify if this entity correspond with our query
        match npc_query.get(ev.entity) {
            Ok(npc) => {
                info!("add detection aura with an npc");

                commands
                    .entity(npc)
                    .insert(DetectionBehavior)
                    .with_children(|parent| {
                        parent
                            .spawn()
                            .insert(Collider::ball(40.))
                            .insert(ActiveEvents::COLLISION_EVENTS)
                            .insert(Sensor)
                            .insert(DetectionSensor)
                            .insert(Name::new("Detection Range"));
                    });
            }

            _ => continue
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

// pub fn old_threat_detection(
//     mut commands: Commands,
//     rapier_context: Res<RapierContext>,
//     player_query: Query<(Entity, &Name, &Children), With<Player>>,
//     npc_query: Query<
//         (Entity, &Name, &Children),
//         (With<DetectionBehavior>, Without<PursuitBehavior>, Without<FairPlayTimer>)>,
//     detection_sensor_query: Query<
//         Entity,
//         (With<DetectionSensor>, Without<PursuitSensor>)>
// ) {
//     let (player, player_name, player_colliders) = player_query.single();
    
//     for (npc, npc_name, npc_colliders) in npc_query.iter() {

//         // info!(target: "Entities checked", "{}", npc_name);
//         // TODO Exclude the pursuit sensor
//         for &npc_child in npc_colliders.iter() {

//             match detection_sensor_query.get(npc_child) {
//                 Ok(sensor) => {
//                     for &player_child in player_colliders.iter() {
                
//                         /* Find the intersection pair, if it exists, between two colliders. */
//                         if rapier_context.intersection_pair(sensor, player_child) == Some(true) {
//                             info!(target: "Threat Detected", "{:?} {} chase {:?} {}", npc, npc_name, player, player_name);
//                         }
//                     }
//                 }
//                 // Err(e)
//                 _ => continue
//             }

//             for &player_child in player_colliders.iter() {
                
//                 /* Find the intersection pair, if it exists, between two colliders. */
//                 if rapier_context.intersection_pair(npc_child, player_child) == Some(true) {
//                     info!(target: "Threat Detected", "{:?} {} chase {:?} {}", npc, npc_name, player, player_name);
                    
//                     // start: NPCDetectionEvent
//                     commands
//                         .entity(npc)
//                         .insert(PursuitBehavior)
//                         .insert(Target(Some(player)));

//                     commands
//                         .entity(npc)
//                         .remove::<DetectionBehavior>();
                    

//                     // if npc_child has PursuitSensor
//                     // turn it true
//                     commands
//                         .entity(npc)
//                         .with_children(|parent| {
//                             parent
//                                 .spawn()
//                                 .insert(Collider::ball(80.))
//                                 .insert(ActiveEvents::COLLISION_EVENTS)
//                                 .insert(Sensor)
//                                 .insert(PursuitSensor)
//                                 .insert(Name::new("Pursuit Range"));
//                         });

//                     // end: NPCDetectionEvent

//                     // TODO give same orders to everyone in the group

//                     // exit the search on the first occurence
//                 }
//             }
//         }
//     }
// }