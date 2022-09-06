use bevy::{prelude::*, ecs::schedule::ShouldRun};

pub mod stats;

use crate::{
    GameState,
    combat::stats::*
};



pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(GameState::Combat)
                    .with_system(roll_initiative)
            )
            .add_system_set_to_stage(
                CoreStage::PostUpdate,
                SystemSet::new()
                    .with_run_criteria(run_if_pressed_h)
                    .with_system(show_hp)
                    .with_system(show_mana)
            )
            ;
    }
}

fn run_if_pressed_h(
    keyboard_input: Res<Input<KeyCode>>
)-> ShouldRun {
    if keyboard_input.just_pressed(KeyCode::H) {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

#[derive(Clone, Copy, Component)]
pub struct Target(pub Option<Entity>);

impl Default for Target {
    fn default() -> Self {
        Target { 0: None }
    }
}

/// The team an entity is assigned to.
#[derive(Copy, Clone, PartialEq, Eq, Component)]
pub struct Team(pub i32);
