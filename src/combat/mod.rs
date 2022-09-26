use bevy::{
    ecs::schedule::ShouldRun,
    time::FixedTimestep, 
    prelude::*,
};

pub mod stats;

use crate::{
    combat::stats::*,
    constants::FIXED_TIME_STEP,
};

#[derive(PartialEq, Clone, Hash, Debug, Eq, SystemLabel)]
enum CombatState {
    Observation,
    SetUpStuff,
    SelectionSkills,
    SelectionTarget,
    RollInitiative,
    ExecuteSkills,
    // ShowExecution,
}

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_system_to_stage(
            //     CoreStage::Update,
            //     stats::roll_initiative
            //         .with_run_criteria(FixedTimestep::step(FIXED_TIME_STEP as f64))
            //         .label(CombatState::RollInitiative)
            // )
            .add_system_to_stage(
                CoreStage::Update,
                observation
                    .with_run_criteria(FixedTimestep::step(FIXED_TIME_STEP as f64))
                    .label(CombatState::Observation)
            )
            // .add_system_set_to_stage(
            //     CoreStage::PostUpdate,
            //     SystemSet::new()
            //         .with_run_criteria(run_if_pressed_h)
            //         .with_system(show_hp)
            //         .with_system(show_mana)
            // )
            ;
    }
}

fn observation(){
    // println!("Now it's your turn...")
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
pub struct Leader;

// #[derive(Clone, Copy, Component)]
// pub struct Target(pub Option<Entity>);

// impl Default for Target {
//     fn default() -> Self {
//         Target { 0: None }
//     }
// }

/// The team an entity is assigned to.
#[derive(Copy, Clone, PartialEq, Eq, Component)]
pub struct Team(pub i32);
