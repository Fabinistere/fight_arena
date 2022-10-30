//! Combat Implementation
//! 
//! Handle
//!   - Combat Initialisation
//!   - Comabt System / Phases
//!     - Stand On
//!     - Open HUD
//!       - Display potential npc's catchphrase (*opening*)
//!       - Display Answers Choices
//!     - Select Approach in the HUD
//!       - talk
//!         - Initialize dialogue
//!       - fight
//! 
//!         ```mermaid
//!         graph
//!             Observation-->ManageStuff;
//!             ManageStuff-->Observation;
//!             Observation-->Skills;
//!             Skills-->Observation;
//!             Skills-->Target;
//!             Target-->Skills;
//!             Target-->RollInitiative;
//!             RollInitiative-->Target;
//!             RollInitiative-->ExecuteSkills-->RollInitiative;
//!             ExecuteSkills-->Observation;
//!         ```
//! 
//!     - Reward-s (gift or loot)
//!   - Combat Evasion (quit)
//! 

use bevy::{
    // ecs::schedule::ShouldRun,
    time::FixedTimestep, 
    prelude::*,
};
use bevy_rapier2d::prelude::Velocity;
use std::time::Duration;

pub mod stats;

use crate::{
    // combat::stats::*,
    // combat::stats::{show_hp, show_mana}
    constants::{
        character::npc::movement::EVASION_TIMER,
        FIXED_TIME_STEP,
    },
    
    npc::{
        aggression::{
            CombatEvent,
            CombatExitEvent,
        },
        NPC,
    },
    player::Player,
};

/// Just help to create a ordered system in the app builder
#[derive(PartialEq, Clone, Hash, Debug, Eq, SystemLabel)]
enum CombatState {
    Initiation,
    Observation,
    // ManageStuff,
    // SelectionSkills,
    // SelectionTarget,
    // RollInitiative,
    // ExecuteSkills,

    // ShowExecution,
    Evasion
}
#[derive(Component)]
pub struct InCombat;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SpawnCombatFoesEvent>()
            .add_system(spawn_party_members.before(CombatState::Initiation))
            .add_system_to_stage(
                CoreStage::Update,
                enter_combat
                    .with_run_criteria(FixedTimestep::step(FIXED_TIME_STEP as f64))
                    .label(CombatState::Initiation)
            )
            .add_system_to_stage(
                CoreStage::Update,
                exit_combat
                    .with_run_criteria(FixedTimestep::step(FIXED_TIME_STEP as f64))
                    .label(CombatState::Evasion)
                    .before(CombatState::Observation)
            )
            .add_system_to_stage(
                CoreStage::Update,
                freeze_in_combat
                    .with_run_criteria(FixedTimestep::step(FIXED_TIME_STEP as f64))
                    .after(CombatState::Evasion)
            )
            .add_system_to_stage(
                CoreStage::Update,
                observation
                    .with_run_criteria(FixedTimestep::step(FIXED_TIME_STEP as f64))
                    .label(CombatState::Observation)
                    .after(CombatState::Initiation)
            )
            // .add_system_to_stage(
            //     CoreStage::Update,
            //     stats::roll_initiative
            //         .with_run_criteria(FixedTimestep::step(FIXED_TIME_STEP as f64))
            //         .label(CombatState::RollInitiative)
            // )
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

#[derive(Clone, Copy, Component)]
pub struct Leader;

/// The team an entity is assigned to.
#[derive(Copy, Clone, PartialEq, Eq, Component)]
pub struct Team(pub i32);

/// One aggressive npc can hide 5 others.
/// This number exclude the 'leader'/representant of the grp
/// 
/// - Could Give info on the type of group ?
///   - (All fabicurion or else)
/// 
/// Min = 0
/// Max = 5
/// 
/// Examples :
/// 
/// - Fabicurion who represent a group of 3
/// - Fabicurion who represent a group of 6
#[derive(Copy, Clone, PartialEq, Eq, Component)]
pub struct GroupSize(pub i32);

/// maybe doublon with GroupSize,
/// must include how much foes are involved to enumerate them
#[derive(Copy, Clone, PartialEq, Eq, Component)]
pub struct GroupType(pub i32);

/// The player can recruted some friendly npc
/// Can be called, TeamPlayer
#[derive(Copy, Clone, PartialEq, Eq, Component)]
pub struct Recruted;

#[derive(Component)]
pub struct FairPlayTimer {
    /// (non-repeating timer)
    /// Let the enemy go when reached/left behind
    pub timer: Timer,
}

/// Happens when:
///   - combat::mod::combat
///     - A aggressive npc encountered the player's group
/// Read in:
///   - combat::mod::spawn_party_members
///     - Spawn every foes hidden behind the initial
///       aggressive npc
pub struct SpawnCombatFoesEvent {
    pub leader: Entity,
    pub group_size: i32
}


/// Emulate the Combat phase
/// 
///   - Talk
///   - Fight
/// 
/// Freeze all entity involved
/// 
///   - Player
///     - all companie members (recruted)
///   - Foe who caught player
pub fn enter_combat(
    mut commands: Commands,

    mut ev_combat_enter: EventReader<CombatEvent>,
    mut ev_spawn_fabicurion: EventWriter<SpawnCombatFoesEvent>,

    mut player_query: Query<
        Entity,
        // must implied the disjunction with player_compagnie
        (With<Player>, Without<NPC>)
    >,
    mut player_companie: Query<
        Entity,
        (With<NPC>, With<Recruted>)
    >,
    mut foes_query: Query<(Entity, &GroupSize), (With<NPC>, Without<Recruted>)>,
) {
    

    for ev in ev_combat_enter.iter() {

        let player = player_query.single_mut() ;

        commands.entity(player)
                .insert(InCombat);
    
        for member in player_companie.iter_mut() {
            commands.entity(member)
                    .insert(InCombat);
   
            // display / spawn them in the ui (CANCELED)
        }

        let npc = ev.npc_entity;

        match foes_query.get_mut(npc) {

            Ok((foe, group_size)) => {

                commands.entity(foe)
                        .insert(InCombat);

                // could be a assert ?
                // no the error could happend cause of human error
                // not an assert matter so. A Require instead
                if group_size.0 < 0 || group_size.0 > 5 {
                    warn!("GroupSize in invalid: < 0 || > 5");
                    // Raise Err ?
                }
                else {
                    ev_spawn_fabicurion.send(
                        SpawnCombatFoesEvent {
                            leader: foe,
                            group_size: group_size.0
                        }
                    );
                }

                // display / spawn them in the ui
                // or
                // spawn them in the temple during combat (PREFERED)
            },

            // Err(e)
            _ => continue

        }

    }
}

/// For each entity in combat, freeze their movement
pub fn freeze_in_combat(
    mut characters_query: Query<
        (Entity, &mut Velocity),
        With<InCombat>
    >,
) {

    // QUESTION: Maybe be not for the member of the company
    // to let them reach the player

    for (_character, mut rb_vel) in characters_query.iter_mut() {
        rb_vel.linvel.x = 0.;
        rb_vel.linvel.y = 0.;
    }
}

/// Event Handler of SpawnCombatFoesEvent
pub fn spawn_party_members(
    mut commands: Commands,
    
    mut ev_spawn_party_members: EventReader<SpawnCombatFoesEvent>
) {
    for ev in ev_spawn_party_members.iter() {
        // ev.group_size
    }
}

/// exit Combat by pressing 'o'
/// 
/// apply to all npc involved in a interaction the IdleBehavior
pub fn exit_combat(
    mut commands: Commands,

    allies_query: Query<
        (Entity, &Name),
        (
            Or<(
                With<Player>,
                (With<NPC>, With<Recruted>)
            )>,
            With<InCombat>
        )
    >,

    foes_query: Query<
        (Entity, &Name),
        (With<NPC>, With<InCombat>, Without<Recruted>)>,

    mut ev_combat_exit: EventReader<CombatExitEvent>,
){

    
    for _ev in ev_combat_exit.iter() {
        for (allie, _name) in allies_query.iter() {
            commands
                .entity(allie)
                .remove::<InCombat>();
        }
        
        // TODO foes AND being an enemy
        for (foes, _name) in foes_query.iter() {
            commands
                .entity(foes)
                .insert(FairPlayTimer { timer: Timer::new(Duration::from_secs(EVASION_TIMER), false)});

            commands
                .entity(foes)
                .remove::<InCombat>();
        }
}

    
}
