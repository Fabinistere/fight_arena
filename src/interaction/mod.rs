use bevy::{ecs::schedule::ShouldRun, prelude::*};

use crate::GameState;

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(choose_interaction.in_schedule(OnEnter(GameState::Interaction)));
    }
}

fn choose_interaction(
    keyboard_input: Res<Input<KeyCode>>,
    // mut player_query: Query<(&Speed, &mut Velocity), With<Player>>,
) {
    println!("Choose TALK or FIGHT");
}
