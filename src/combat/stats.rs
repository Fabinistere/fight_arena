//! Implement all Combat stats 

use bevy::prelude::*;
use rand::Rng;

use crate::{
    npc::NPC,
    player::Player
};

/// Each entity which can be involved in a combat has this Bundle
#[derive(Bundle)]
pub struct CombatBundle {
    pub hp: HP,
    pub mana: MANA,
    pub initiative: Initiative,
    pub attack: Attack,
    pub attack_spe: AttackSpe,
    pub defense: Defense,
    pub defense_spe: DefenseSpe
}


/// ----------HP----------
/// Start of the Game: 50hp -> End of the Game: 1 000hp
/// Can be modified by level, item, buff, debuff, technics.
/// At the moment, current_hp <= max_hp
#[derive(Component)]
pub struct HP {
    pub current_hp: i32,
    pub max_hp: i32
}

impl Default for HP {
    fn default() -> Self {
        HP {
            current_hp: 50,
            max_hp: 50
        }
    }
}

// TODO a hp bar close to the entity
pub fn show_hp(
    player_query: Query<&HP, With<Player>>,
    npc_query: Query<(&HP, &Name), With<NPC>>,
){
    let player_hp = player_query.single();
    for (npc_hp, npc_name) in npc_query.iter() {
        println!(
            "{}'s HP: {}/{},", 
            npc_name, npc_hp.current_hp, npc_hp.max_hp);
    }

   println!(
    "player's HP: {}/{}",
    player_hp.current_hp, player_hp.max_hp);
}

/// ----------MANA----------
/// Start of the Game: 0-100mana -> End of the Game: 10 000mana
/// Can be modified by level, item, buff, debuff, technics.
/// At the moment, current_mana <= max_mana
#[derive(Component)]
pub struct MANA {
    pub current_mana: i32,
    pub max_mana: i32
}

impl Default for MANA {
    fn default() -> Self {
        MANA {
            current_mana: 50,
            max_mana: 50
        }
    }
}

// TODO a mana bar close to the entity
pub fn show_mana(
    player_query: Query<&MANA, With<Player>>,
    npc_query: Query<(&MANA, &Name), With<NPC>>,
){
    let player_mana = player_query.single();
    for (npc_mana, npc_name) in npc_query.iter() {
        println!(
            "{}'s MANA: {}/{},", 
            npc_name, npc_mana.current_mana, npc_mana.max_mana);
    }

   println!(
    "player's MANA: {}/{}",
    player_mana.current_mana, player_mana.max_mana);
}

/// ----------Attack----------
/// Start of the Game: 10-20 -> End of the Game: ~
/// Can be modified by level, item, buff, debuff, technics.
/// This statistic is fix, it increment the martial technic's power.
#[derive(Component)]
pub struct Attack {
    pub attack: i32
}

impl Default for Attack {
    fn default() -> Self {
        Attack {
            attack: 10
        }
    }
}

/// ----------Attack Spe----------
/// Start of the Game: 0-30 -> End of the Game: ~
/// Can be modified by level, item, buff, debuff, technics.
/// This statistic is fix, it increment the magic technic's power.
#[derive(Component)]
pub struct AttackSpe {
    pub attack_spe: i32
}

impl Default for AttackSpe {
    fn default() -> Self {
        AttackSpe {
            attack_spe: 0
        }
    }
}

/// ----------Defense----------
/// Start of the Game: 0-10 -> End of the Game: ~
/// Can be modified by level, item, buff, debuff, technics.
/// This statistic has a logarithmic behavior.
/// Used to calculate the reduced damage (in percentage)
/// taken from basic attacks and abilities that deal physical damage.
/// Calculated by armor ÷ (armor + 100).
#[derive(Component)]
pub struct Defense {
    pub defense: i32
}

impl Default for Defense {
    fn default() -> Self {
        Defense {
            defense: 10
        }
    }
}

/// ----------Defense Spe----------
/// Start of the Game: 0-10 -> End of the Game: ~
/// Can be modified by level, item, buff, debuff, technics.
/// This statistic has a logarithmic behavior.
/// Used to calculate the reduced damage (in percentage)
/// taken from basic attacks and abilities that deal magical damage.
/// Calculated by MR ÷ (MR + 100).
#[derive(Component)]
pub struct DefenseSpe {
    pub defense_spe: i32
}

impl Default for DefenseSpe {
    fn default() -> Self {
        DefenseSpe {
            defense_spe: 0
        }
    }
}

/// ----------INITIATIVE----------
/// Minimun initiative: 0 -> Maximun initiative: 100
/// Indicate the speed of initiative, the entity has.
/// The more he has, the more likly he will start his turn first.
#[derive(Component)]
pub struct Initiative {
    pub initiative: i32
}

impl Default for Initiative {
    fn default() -> Self {
        Initiative {
            initiative: 20
        }
    }
}

/// Roll for each entity a d100 ranged into +-20 initiative
/// Sort the result in a nice table
/// In case of egality: pick the higher initiative boyo to be on top
pub fn roll_initiative(
    player_query: Query<&Initiative, With<Player>>,
    npc_query: Query<&Initiative, With<NPC>>
) {
    let mut v: Vec<i32> = Vec::new();

    let player_init = player_query.single();
    for npc_init in npc_query.iter() {
        let npc_number;

        if npc_init.initiative -20 <= 0 {
            npc_number =
            rand::thread_rng().gen_range(0..npc_init.initiative +20);
        } else if npc_init.initiative == 100{
            npc_number = 100;
        } else if npc_init.initiative +20 >= 100 {
            npc_number =
            rand::thread_rng().gen_range(npc_init.initiative -20..100);
        } else {
            npc_number =
            rand::thread_rng().gen_range(npc_init.initiative -20..npc_init.initiative +20);
        }

        // insert these number in a vector
        v.push(npc_number);

    }

    let player_number;

    if player_init.initiative -20 <= 0 {
        player_number =
        rand::thread_rng().gen_range(0..player_init.initiative +20);
    } else if player_init.initiative == 100{
        player_number = 100;
    } else if player_init.initiative +20 >= 100 {
        player_number =
        rand::thread_rng().gen_range(player_init.initiative -20..100);
    } else {
        player_number =
        rand::thread_rng().gen_range(player_init.initiative -20..player_init.initiative +20);
    }

    v.push(player_number);

    v.sort();

    println!("{:?}", v);
    
}

/// ----------ACCURACY----------
/// Used to calculate if the technic will hit (in percentage).
#[derive(Component)]
pub struct Accuracy {
    pub accuracy: i32
}

impl Default for Accuracy {
    fn default() -> Self {
        Accuracy {
            accuracy: 95
        }
    }
}

/// ----------CRITICAL----------
/// Used to calculate if the technic will be critical (in percentage).
/// A Critical technic has its dmg inflicted multiplied by 300%
/// ONLY allow critics on hit
#[derive(Component)]
pub struct Critical {
    pub critical: i32
}

impl Default for Critical {
    fn default() -> Self {
        Critical {
            critical: 1
        }
    }
}