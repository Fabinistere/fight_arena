use bevy::prelude::*;
use crate::npc::movement::{
    RunToDestinationBehavior,
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
        (Entity, &mut RestTime), 
        (With<IdleBehavior>, Without<FollowBehavior>)
    >
) {
    for (npc, mut rest_timer) in npc_query.iter_mut() {

        rest_timer.timer.tick(time.delta());

        // println!("npc's state: {:#?}", npc.state);

        // flexing animation

        // println!(
        //     "{} is resting, rem time: {}",
        //     npc.name,
        //     rest_timer.timer.elapsed_secs()
        // );                      

        // TODO check it.
        // When ONE timer is finished
        // all npc goes to work (walk)
        // the others timer still live
        if rest_timer.timer.finished() {

            commands.entity(npc)
                    .insert(
                        RunToDestinationBehavior {
                            destination: give_a_direction()
                    });
            commands.entity(npc)
                    .remove::<RestTime>();
        }          

    }
}