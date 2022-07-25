use bevy::prelude::*;
use rand::Rng;

use crate::{FabienSheet, TILE_SIZE};
use crate::spawn_fabien_sprite;

pub struct NPCPlugin;

#[derive(Component)] // Inspectable
pub struct NPC {
    speed: f32
}

impl Plugin  for NPCPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_character)
            .add_system(character_movement);

    }
}

fn character_movement(
    mut npc_query: Query<(&mut NPC, &mut Transform)>,
    time: Res<Time>,
){
    // like my brain empty

    let (npc, mut transform) = npc_query.single_mut();

    let direction = give_a_direction();

    // if direction ... {}
    // transform.translation.y += npc.speed * TILE_SIZE * time.delta_seconds();
    // transform.translation.x += npc.speed * TILE_SIZE * time.delta_seconds();
}

/**
 * param:
 *  force
 *  range: cuboid ?
 * return:
 *  Vec3
 */
fn give_a_direction() -> Vec3
{
    let x = (rand::thread_rng().gen_range(-10..10) / 10) as f32 ;
    let y = (rand::thread_rng().gen_range(-10..10) / 10)  as f32;
    // let z = rand::thread_rng().gen_range(1..101);

    let direction = Vec3::new(x, y, 0.0);

    direction
}


fn spawn_character(
    mut commands: Commands,
    fabien: Res<FabienSheet>
)
{
    let character = spawn_fabien_sprite(
        &mut commands,
        &fabien,
        0,
        Color::rgb(0.9,0.9,0.9),
        Vec3::new(-0.2, 0.35, 5.0),
        Vec3::new(2.0,2.0,0.0)
    );

    commands
        .entity(character)
        .insert(Name::new("NPC"))
        .insert(NPC { speed: 3.0 });
}