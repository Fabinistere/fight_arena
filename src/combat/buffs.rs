//! Implement all Combat Buffs and Debuffs 

use bevy::prelude::*;

// insert Combat Bundle / create buffBundle stats
#[derive(Component)]
struct Buffs {
    description: String,
    turn: i32,
}