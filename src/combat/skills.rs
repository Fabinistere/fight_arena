//! Implement SKILLS

use crate::combat::stats::*;
use bevy::prelude::*;

/// - Negative = MALUS
/// - Positive = BONUS
#[derive(Component)]
pub struct Skill {
    /// hp: dmg; heal
    pub hp: Hp,
    /// mana: consume; gain
    pub mana: Mana,
    /// initiave: slower; faster
    pub initiative: Initiative,
    /// att: lose; gain
    pub attack: Attack,
    /// att spe: lose; gain
    pub attack_spe: AttackSpe,
    /// def: lose; gain
    pub defense: Defense,
    /// def spe: lose; gain
    pub defense_spe: DefenseSpe,
    /// The 'list' of skills called after this one
    pub skills_queue: Vec<Skill>,
    pub description: String,
}

fn skill_caller(query: Query<(Entity, &Skill)>, // ??
) {
}
