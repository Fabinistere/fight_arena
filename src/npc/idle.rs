use bevy::prelude::*;
use crate::npc::movement::{
    JustWalkBehavior,
    FollowBehavior,
    give_a_direction
};

#[derive(Component)]
pub struct IdleBehavior;

#[derive(Component)]
pub struct RestTime {
    /// track when the npc should stop rest (non-repeating timer)
    pub timer: Timer,
}

pub fn do_flexing(
    mut commands: Commands,
    time: Res<Time>,
    mut npc_query: Query<
        (Entity, &mut RestTime, &Name), 
        (With<IdleBehavior>, Without<FollowBehavior>)
    >
) {
    for (npc, mut rest_timer, name) in npc_query.iter_mut() {

        rest_timer.timer.tick(time.delta());

        // flexing animation                 

        if rest_timer.timer.finished() {

            println!(
                "{} got a way to go",
                name
            );

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
//         (With<IdleBehavior>, With<FollowBehavior>)
//     >
// ) {
//     for (npc, name) in npc_query.iter_mut() {

//         // flexing animation                 

//         commands.entity(npc)
//                 .insert(
//                     FollowBehavior);
//         commands.entity(npc)
//                 .remove::<IdleBehavior>();      

//     }
// }