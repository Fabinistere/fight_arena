use bevy::prelude::*;
use rand::Rng;
use std::time::Duration;

use crate::{FabienSheet, TILE_SIZE};
use crate::spawn_fabien_sprite;

pub struct NPCPlugin;

#[derive(Component)] // Inspectable
pub struct NPC {
    speed: f32
}

enum NPCState {
    Running,
    Following,
    Rest,
    Talking
}

impl Plugin  for NPCPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_character)
            .add_system(character_movement)
            ;

    }
}

#[derive(Component)]
struct FuseTime {
    /// track when the npc should stop rest (non-repeating timer)
    timer: Timer,
}

/// repos: timer
fn character_movement(
    mut npc_query: Query<(&mut NPC, &mut Transform)>,
    time: Res<Time>,
){

    // , &mut FuseTime)>
    

    // like my brain BIG but not so usefull (lie?)

    let (npc, mut transform) = npc_query.single_mut();

    let direction = give_a_direction();
    println!("npc just got a way to go");
    println!("x: {} y: {} z: {}", direction.x, direction.y, direction.z);
    // destination.y

    /*
    while direction.y != transform.translation.y &&
          direction.x != transform.translation.x {

        if direction.y > transform.translation.y {
            transform.translation.y += npc.speed * TILE_SIZE * time.delta_seconds();
        }
        
        if direction.y < transform.translation.y {
            transform.translation.y -= npc.speed * TILE_SIZE * time.delta_seconds();
        }

        if direction.x > transform.translation.x {
            transform.translation.x += npc.speed * TILE_SIZE * time.delta_seconds();
        }
        
        if direction.x < transform.translation.x {
            transform.translation.x -= npc.speed * TILE_SIZE * time.delta_seconds();
        }
    }

     */

    // insert state: Rest

    println!("npc just wanna rest");

    // commands.entity(npc)
    //         .insert(FuseTime {
    //             // create the non-repeating fuse timer
    //             timer: Timer::new(Duration::from_secs(5), false),
    //         });

    // for (_, _, mut fuse_timer) in npc_query.iter_mut() {
    //     // timers gotta be ticked, to work
    //     fuse_timer.timer.tick(time.delta());

    //     if fuse_timer.timer.finished() {
    //         // consider this as death
    //         commands.entity(npc).despawn();
    //         println!("npc just despawn");
    //     }

    // }

    // https://bevy-cheatbook.github.io/features/time.html

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
    let x = rand::thread_rng().gen_range(-10..10) as f32 / 10.0;
    let y = rand::thread_rng().gen_range(-10..10) as f32 / 10.0;
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