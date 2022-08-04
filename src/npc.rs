use bevy::prelude::*;
use rand::Rng;
use std::time::Duration;

use crate::{FabienSheet, TILE_SIZE};
use crate::spawn_fabien_sprite;

pub struct NPCPlugin;

#[derive(Component)] // Inspectable
pub struct NPC {
    // check if we can remove this field and replace it by just get_label()
    name: String,
    speed: f32,
    direction: Vec3,
    state: NPCState
}

// State for individual entity ?!
// without using the bevy state tool
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum NPCState {
    Running,
    // Following,
    Rest,
    // Talking
}

/**
 * NPC has hobbies
 *  - landwark
 *    - index in const, with free: bol
 *    - when talking to a npc in a landwark, include the other present
 *    -> rest
 *  - stroll
 *    - in a restricted zone -index in const-
 *    -> rest
 *  - rest
 *    -> stroll
 *    -> landwark
 *  - talking to MC
 *    - infite rest until the MC is leaving
 *    -> short rest
 *    or
 *    -> stroll
 *    -> landmark
 *    -> rest
 * 
 * Reflexion
 *  - should npc avoid hit other entity
 *  - turn false the free param from a landmark position taken by the MC
 */
impl Plugin  for NPCPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state(NPCState::Running)
            .add_startup_system(spawn_character)
            .insert_resource(RestTime {
                timer: Timer::from_seconds(10.0, false)
            })
            .add_system(stroll)
            .add_system(rest)
            ;
    }
}

fn stroll(
    mut npc_query: Query<(&mut NPC, &mut Transform)>,
    time: Res<Time>,
    mut npc_state: ResMut<State<NPCState>>,
    mut commands: Commands,
) {
    for (mut npc, mut transform) in npc_query.iter_mut() {

        // TODO remove this conditon, and find a better solution
        if npc.state == NPCState::Running {

            let direction = npc.direction;

            // TODO Approximation Louche
            if ((direction.y*100.0) as i32 != (transform.translation.y*100.0) as i32     &&
                (direction.y*100.0) as i32 != (transform.translation.y*100.0) as i32 + 1 &&
                (direction.y*100.0) as i32 != (transform.translation.y*100.0) as i32 - 1)
                ||
               ((direction.x*100.0) as i32 != (transform.translation.x*100.0) as i32     &&
                (direction.x*100.0) as i32 != (transform.translation.x*100.0) as i32 + 1 &&
                (direction.x*100.0) as i32 != (transform.translation.x*100.0) as i32 - 1)
            {

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
            } else {
                println!(
                    "I'm {} and I'm gonna rest for a while",
                    npc.name
                );
                npc.state = NPCState::Rest;
                // ne sert a R atm
                commands.spawn()
                        .insert(RestTime {
                            // create the non-repeating fuse timer
                            timer: Timer::new(Duration::from_secs(10), false),
                        });
            }

            // println!(
            //     "direction: {}, {} pos: {}
            //     \n pos+1: [{}, {}, {}]
            //     \n pos-1: [{}, {}, {}]",
            //     direction,
            //     npc.name,
            //     transform.translation,
            //     (transform.translation.x*100.0) as i32 +1,
            //     (transform.translation.y*100.0) as i32 +1,
            //     transform.translation.z,
            //     (transform.translation.x*100.0) as i32 -1,
            //     (transform.translation.y*100.0) as i32 -1,
            //     transform.translation.z,

            // );

            /*
             direction: [-1, 0.8, 0], Admiral pos: [-1.0008445, 0.79996014, 5]
             direction: [0.3, -1, 0], Olf pos: [0.29992545, -1.0007071, 5]
            */

        }
    }
}

fn rest(
    time: Res<Time>,
    mut npc_query: Query<(&mut NPC, &mut RestTime)>,
) {

    for (mut npc, mut rest_time) in npc_query.iter_mut() {

        if npc.state == NPCState::Rest {
            // flexing animation
    
            println!("{} is resting", npc.name);
            rest_time.timer.tick(time.delta());
            println!("rest time: {}", rest_time.timer.elapsed_secs());
    
            if rest_time.timer.finished() {
                npc.state = NPCState::Running;
                npc.direction = give_a_direction();
            }
        }
    }
}
            

#[derive(Component)]
struct RestTime {
    /// track when the npc should stop rest (non-repeating timer)
    timer: Timer,
}

/// Configure our resting time algorithm
fn setup_rest_time(
    mut commands: Commands,
) {
    commands.insert_resource(RestTime {
        // create the repeating timer
        timer: Timer::new(Duration::from_secs(10), true),
    })
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
    let position = Vec3::new(-0.2, 0.35, 5.0);

    let admiral = spawn_fabien_sprite(
        &mut commands,
        &fabien,
        0,
        Color::rgb(0.9,0.9,0.9),
        position,
        Vec3::new(2.0,2.0,0.0)
    );

    let olf = spawn_fabien_sprite(
        &mut commands,
        &fabien,
        12,
        Color::rgb(0.9,0.9,0.9),
        Vec3::new(-0.2, 0.55, 5.0),
        Vec3::new(2.0,2.0,0.0)
    );

    commands
        .entity(admiral)
        .insert(Name::new("NPC Admiral"))
        .insert(
        NPC {
            name: "Admiral".to_string(),
            speed: 3.0,
            direction: give_a_direction(),
            state: NPCState::Running
        });

    commands
        .entity(olf)
        .insert(Name::new("NPC Olf"))
        .insert(
        NPC {
            name: "Olf".to_string(),
            speed: 3.0,
            direction: give_a_direction(),
            state: NPCState::Running
        });
}