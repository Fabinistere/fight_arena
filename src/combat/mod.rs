use std::time::Duration;

use bevy::{
    // ecs::schedule::ShouldRun,
    time::FixedTimestep, 
    prelude::*,
};

pub mod stats;

use crate::{
    // combat::stats::*,
    constants::{
        character::npc::movement::EVASION_TIMER,
        FIXED_TIME_STEP,
    },
    npc::{
        aggression::CombatExitEvent,
        NPC,
    },
    // combat::stats::{show_hp, show_mana}
};

#[derive(PartialEq, Clone, Hash, Debug, Eq, SystemLabel)]
enum CombatState {
    Observation,
    // SetUpStuff,
    // SelectionSkills,
    // SelectionTarget,
    // RollInitiative,
    // ExecuteSkills,
    // ShowExecution,
    
    // is this a good idea ?
    Evasion
}
#[derive(Component)]
pub struct InCombat;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_to_stage(
                CoreStage::Update,
                exit_combat
                    .with_run_criteria(FixedTimestep::step(FIXED_TIME_STEP as f64))
                    .label(CombatState::Evasion)
            )
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

// fn run_if_pressed_h(
//     keyboard_input: Res<Input<KeyCode>>
// )-> ShouldRun {
//     if keyboard_input.just_pressed(KeyCode::H) {
//         ShouldRun::Yes
//     } else {
//         ShouldRun::No
//     }
// }

#[derive(Clone, Copy, Component)]
pub struct Leader;

/// The team an entity is assigned to.
#[derive(Copy, Clone, PartialEq, Eq, Component)]
pub struct Team(pub i32);

#[derive(Component)]
pub struct FairPlayTimer {
    /// (non-repeating timer)
    /// Let the enemy go when reached/left behind
    pub timer: Timer,
}

/// exit Combat by pressing 'o'
/// 
/// apply to all npc involved in a interaction the IdleBehavior
pub fn exit_combat(
    mut commands: Commands,
    npc_query: Query<
        (Entity, &Name),
        (With<NPC>, With<InCombat>)>,
    mut ev_combat_exit: EventWriter<CombatExitEvent>,
    keyboard_input: Res<Input<KeyCode>>,
){
    if keyboard_input.just_pressed(KeyCode::O) {
        // TODO npc AND being an enemy
        for (npc, _name) in npc_query.iter() {
            commands.entity(npc)
                .insert(FairPlayTimer { timer: Timer::new(Duration::from_secs(EVASION_TIMER), false)});
        }

        ev_combat_exit.send(CombatExitEvent);
    }

    
}
