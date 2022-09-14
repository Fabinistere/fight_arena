//! Implement SKILLS

use crate::{
    combat::stats::*,

};

/// Negative = MALUS
/// Positive = BONUS
/// 
/// Skills :
/// hp: heal; dmg
/// mana: gain; consume
/// initiave: faster; slower
/// att/def/spe: gain; lose
pub struct Skills {
    pub hp: HP,
    pub mana: MANA,
    pub initiative: Initiative,
    pub attack: Attack,
    pub attack_spe: AttackSpe,
    pub defense: Defense,
    pub defense_spe: DefenseSpe,
    pub skills_queue: Vec![Skills],
    pub description: String
}

fn skill_caller(
    query: Query<(&NPC)>, // ??
    skill: Skills,
    target: &Target
){

}