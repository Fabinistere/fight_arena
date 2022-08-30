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
                        RunToDestinationBehavior {
                            destination: give_a_direction()
                    });
            commands.entity(npc)
                    .remove::<IdleBehavior>();
            commands.entity(npc)
                    .remove::<RestTime>();
        }          

    }
}