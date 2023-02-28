use bevy::{prelude::*, ecs::schedule::ShouldRun};

use crate::{
    GameState,
};


pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(GameState::Interaction)
                    .with_system(choose_interaction)
            )
            ;
    }
}

fn choose_interaction(
    keyboard_input: Res<Input<KeyCode>>,
    // mut player_query: Query<(&Speed, &mut Velocity), With<Player>>,
) {
    println!("Choose TALK or FIGHT");
}