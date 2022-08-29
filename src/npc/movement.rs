//! Implements Npc for moving and steering entities.

use bevy::prelude::*;
use std::time::Duration;
use rand::Rng;

// use crate::combat::Target;
use crate::npc::idle::{
    IdleBehavior,
    RestTime
};
use crate::{
    constants::npc::movement::*,
    movement::Speed,
    TILE_SIZE
};

/// Indicates that an entity should run towards a destination.
#[derive(Default, Component)]
pub struct RunToDestinationBehavior {
    pub destination: Vec3,
}

#[derive(Default, Component)]
pub struct FollowBehavior;
// pub const PROXIMITY_RADIUS: f32 = 64.0;

/// Turns entities with a [TurnToDestinationBehavior](TurnToDestinationBehavior.struct.html) towards their destination.
pub fn run_to_destination(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &RunToDestinationBehavior,
        &mut Transform,
        &Speed
    )>,
    time: Res<Time>,
) {
    for (npc, behavior, mut transform, speed) in query.iter_mut() {
        let direction: Vec3 = behavior.destination;

        // TODO Approximation Louche
        if !close(transform.translation, direction)
        {

            // TODO : Use viscosity (rapier) instead of teleportation

            if direction.y > transform.translation.y {
                transform.translation.y += speed.0 * TILE_SIZE * time.delta_seconds();
            }
        
            if direction.y < transform.translation.y {
                transform.translation.y -= speed.0 * TILE_SIZE * time.delta_seconds();
            }
        
            if direction.x > transform.translation.x {
                transform.translation.x += speed.0 * TILE_SIZE * time.delta_seconds();
            }
        
            if direction.x < transform.translation.x {
                transform.translation.x -= speed.0 * TILE_SIZE * time.delta_seconds();
            }
        } else {
            // println!(
            //     "I'm {} and I'm gonna rest for a while",
            //     npc.type_name()
            // );
            commands.entity(npc)
                    .remove::<RunToDestinationBehavior>();
            commands.entity(npc)
                    .insert(IdleBehavior);
            // println!("postChange: npc's state: {:#?}", npc.state);
            
            commands.entity(npc)
                    .insert(RestTime {
                        // create the non-repeating rest timer
                        timer: Timer::new(Duration::from_secs(REST_TIMER), false),
                    });
        }
    }
    //println!("turn_to_destination: {:?} entities.", query.iter_mut().len());
}

/// Entity pursues their target.
// pub fn follow(
//     mut commands: Commands,
//     mut query: Query<(
//         Entity,
//         &FollowBehavior,
//         &Target,
//         &GlobalTransform,
//         &RunToDestinationBehavior,
//     )>,
//     pos_query: Query<&GlobalTransform>,
// ) {
//     for (npc, _follow, target, transform, run_to) in query.iter_mut() {
        
//         if target.0.is_none() {
//             continue;
//         }

//         let result = pos_query.get_component::<GlobalTransform>(target.0.expect("target is none"));
        
//         match result {
//             Err(_) => {
//                 // target does not have position. Go to idle state
//                 commands.entity(npc).remove::<FollowBehavior>();
//                 commands.entity(npc).insert(IdleBehavior);
//                 continue;
//             }
//             Ok(target_transform) => {
//                 run_to.destination = target_transform.translation();
                
//                 // println!("entity: {:?}, destination: {:?}, delta: {:?}.", target.0.expect("target"), follow.destination, delta);
                
//                 // TODO make the npc not merging with the target
//             }
//         }
//     }
//     // println!("pursue: {:?} entities, {:?} err, {:?} ok.", query.iter_mut().len(), err_count, ok_count);
// }

/**
 * @param
 * position: of a entity
 * direction: the middle of the future zone, 
 *            is on the middle of the segment [a,c]
 * @return true
 * if the entity is on the square around the direction point
 */
fn close(
    position: Vec3,
    direction: Vec3,
) -> bool
{
    // direction.x == position.x &&
    // direction.y == position.y
    
    let a = 
        Vec3::new(
            direction.x-TILE_SIZE/2.0,
            direction.y+TILE_SIZE/2.0,
            direction.z
        );

    let c = 
        Vec3::new(
            direction.x+TILE_SIZE/2.0,
            direction.y-TILE_SIZE/2.0,
            direction.z
        );
    
    position.x >= a.x && position.x <= c.x &&
    position.y <= a.y && position.y >= c.y 
    
}

/**
 * param:
 *  force
 *  range: cuboid ?
 * return:
 *  Vec3
 */
pub fn give_a_direction() -> Vec3
{
    let x = rand::thread_rng().gen_range(-10..10) as f32 / 10.0;
    let y = rand::thread_rng().gen_range(-10..10) as f32 / 10.0;
    // let z = rand::thread_rng().gen_range(1..101);

    /* shape ideas
     * (x, y) -> A
     * (x+1, y-1) -> C
     * (x+0.5, y-0.5) -> milieu
     */

    let direction: Vec3 = Vec3::new(x, y, 0.0);

    direction
}