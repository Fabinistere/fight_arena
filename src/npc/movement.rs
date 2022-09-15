//! Implements Npc for moving and steering entities.

use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;
use log::{info, warn};
use std::time::Duration;
use rand::Rng;

use crate::{
    // combat::Target,
    constants::npc::movement::*,
    movement::Speed,
    npc::idle::{
        IdleBehavior,
        RestTime
    },
    TILE_SIZE,
    combat::{Leader, Team},
    // player::Player
};

use super::NPC;

/// Indicates that an entity should run towards a destination and which.
#[derive(Default, Component)]
pub struct JustWalkBehavior {
    pub destination: Vec3,
}

#[derive(Default, Component)]
pub struct FollowBehavior;
// pub const PROXIMITY_RADIUS: f32 = 64.0;

// TODO use ColliderType::Sensor to delimiter zone

/// For a certain destination contained in [RunToDestinationbehavior], make the npc run towards it
pub fn just_walk(
    mut commands: Commands,
    mut npc_query: Query<(
        Entity,
        &JustWalkBehavior,
        &Transform,
        &Speed,
        &mut Velocity,
        &Name
    ), (With<JustWalkBehavior>, Without<IdleBehavior>)>
) {
    for (npc, behavior, transform, speed, mut rb_vel, name) in npc_query.iter_mut() {
        let direction: Vec3 = behavior.destination;

        // TODO Approximation Louche
        if !close(transform.translation, direction, TILE_SIZE/2.0)
        {

//             println!(
//                 "{} direction: ({},{})
// position: ({},{})",
//                 name, direction.x, direction.y,
//                 transform.translation.x, transform.translation.y
//             );

            let up = direction.y > transform.translation.y;
            let down = direction.y < transform.translation.y;
            let left = direction.x < transform.translation.x;
            let right = direction.x > transform.translation.x;

            let x_axis = -(left as i8) + right as i8;
            let y_axis = -(down as i8) + up as i8;

            // println!("x: {}, y: {}", x_axis, y_axis);
    
            let mut vel_x = x_axis as f32 * **speed;
            let mut vel_y = y_axis as f32 * **speed;
    
            if x_axis != 0 && y_axis != 0 {
                vel_x *= (std::f32::consts::PI / 4.0).cos();
                vel_y *= (std::f32::consts::PI / 4.0).cos();
            }
    
            rb_vel.linvel.x = vel_x;
            rb_vel.linvel.y = vel_y;

        } else {
            
            info!(target: "Start Rest", "{:?}, {}", npc, name);

            // Stop the npc after reaching the destination
            rb_vel.linvel.x = 0.0;
            rb_vel.linvel.y = 0.0;

            commands.entity(npc)
                    .remove::<JustWalkBehavior>();
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
}

/// Entity pursues their target.
pub fn follow(
    // mut commands: Commands,
    mut npc_query: Query<(
        Entity, 
        &Transform,
        &Speed,
        &mut Velocity,
        &Team
    ), (With<NPC>, With<FollowBehavior>) // only npc can follow 
    >,
    targets_query: Query<(&Transform, &Team, &Name), With<Leader>>,
    // pos_query: Query<&GlobalTransform>,
) {
    for (_npc, transform, speed, mut rb_vel, team) in npc_query.iter_mut() {

        for (target_transform, target_team, name) in targets_query.iter() {

            
            // println!("target: {}, Leader of team {}", name, target_team.0);

            // TODO Rework this Approximation Louche
            // carefull with more than one leader per team
            // it will be not nice
            if !close(transform.translation, target_transform.translation, TILE_SIZE*2.0)
                &&
               (team.0 == target_team.0)
            {
                // println!("moving towards target: {}", name);
    
                let up = target_transform.translation.y > transform.translation.y;
                let down = target_transform.translation.y < transform.translation.y;
                let left = target_transform.translation.x < transform.translation.x;
                let right = target_transform.translation.x > transform.translation.x;
    
                let x_axis = -(left as i8) + right as i8;
                let y_axis = -(down as i8) + up as i8;
    
                // println!("x: {}, y: {}", x_axis, y_axis);
        
                let mut vel_x = x_axis as f32 * **speed;
                let mut vel_y = y_axis as f32 * **speed;
        
                if x_axis != 0 && y_axis != 0 {
                    vel_x *= (std::f32::consts::PI / 4.0).cos();
                    vel_y *= (std::f32::consts::PI / 4.0).cos();
                }
        
                rb_vel.linvel.x = vel_x;
                rb_vel.linvel.y = vel_y;
    
            } else {
                // TODO AVOID npc to merge
                rb_vel.linvel.x = 0.0;
                rb_vel.linvel.y = 0.0;
            }
        }
        }
        

        
    // target does not have position. Go to idle state
    // commands.entity(npc).remove::<FollowBehavior>();
    // commands.entity(npc).remove::<RunToDestinationBehavior>();
    // commands.entity(npc).insert(IdleBehavior);

    // println!("pursue: {:?} entities, {:?} err, {:?} ok.", query.iter_mut().len(), err_count, ok_count);
}

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
    range: f32
) -> bool
{
    // direction.x == position.x &&
    // direction.y == position.y
    
    let a = 
        Vec3::new(
            direction.x-range,
            direction.y+range,
            direction.z
        );

    let c = 
        Vec3::new(
            direction.x+range,
            direction.y-range,
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