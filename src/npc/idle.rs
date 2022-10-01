use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;
use log::{
    info,
    // warn
};

use super::movement::{
    JustWalkBehavior,
    FollowupBehavior,
    PursuitBehavior,
    give_a_direction
};

#[derive(Component)]
pub struct IdleBehavior;

#[derive(Component)]
pub struct RestTime {
    /// track when the npc should stop rest (non-repeating timer)
    pub timer: Timer,
}

// TODO Create a starting idleBehavior
// to avoid: 
// - to give a direction in the spawn NPC
// - To give a RestTime in spwan

pub fn do_flexing(
    mut commands: Commands,
    time: Res<Time>,
    mut npc_query: Query<
        (Entity, &mut RestTime, &mut Velocity, &Name), 
        (With<IdleBehavior>, Without<FollowupBehavior>, Without<PursuitBehavior>)
    >
) {
    for (npc, mut rest_timer, mut rb_vel, name) in npc_query.iter_mut() {

        rest_timer.timer.tick(time.delta());

        // prevent npcs from being launched by pushing them
        rb_vel.linvel.x = 0.;
        rb_vel.linvel.y = 0.;

        // flexing animation                 

        if rest_timer.timer.finished() {

            info!(target: "Got a way to go", "{:?}, {}", npc, name);

            commands.entity(npc)
                    .insert(
                        JustWalkBehavior {
                            destination: give_a_direction()
                    });
            commands.entity(npc)
                    .remove::<IdleBehavior>();
            commands.entity(npc)
                    .remove::<RestTime>();
        }          

    }
}

// pub fn wait_leader(
//     mut commands: Commands,
//     mut npc_query: Query<
//         (Entity, &Name), 
//         (With<IdleBehavior>, With<FollowupBehavior>)
//     >
// ) {
//     for (npc, name) in npc_query.iter_mut() {

//         // flexing animation                 

//         commands.entity(npc)
//                 .insert(
//                     FollowupBehavior);
//         commands.entity(npc)
//                 .remove::<IdleBehavior>();      

//     }
// }