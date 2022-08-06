use bevy::{
    prelude::*, ecs::schedule::ShouldRun};
use bevy_rapier2d::prelude::Collider;
use rand::{Rng}; //thread_rng, 
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
// without using the bevy global state tool
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum NPCState {
    Stroll,
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
            // louche
            // .add_state(NPCState::Stroll)
            .add_startup_system(spawn_character)
            // .add_startup_system(show_ieud_grid)
            // .add_system_set(
            //     SystemSet::on_enter(NPCState::Stroll)
            //         .with_system(spawn_destination)
            // )
            // .add_system_set(
            //     SystemSet::new()
            //         .with_run_criteria(run_if_strolling)
            //         .with_system(stroll)
            // )
            // .add_system_set(
            //     SystemSet::new()
            //         .with_run_criteria(run_if_rest)
            //         .with_system(rest)
            // )
            .add_system_set(
                SystemSet::new()
                    .label("stroll")
                    .with_system(stroll)
            )
            .add_system_set(
                SystemSet::new()
                    .label("rest")
                    .before("stroll")
                    .with_system(rest)
            )
            // .insert_resource(RestTime {
            //     timer: Timer::from_seconds(10.0, false)
            // })
            ;
    }
}

fn run_if_strolling(
    mut npc_query: Query<&mut NPC>,
) -> ShouldRun
{
    for npc in npc_query.iter_mut() {
        if npc.state == NPCState::Stroll {
            return ShouldRun::Yes;
        }
    }

    return ShouldRun::No;
}

fn stroll(
    mut npc_query: Query<(&mut NPC, &mut Transform)>,
    time: Res<Time>,
    // mut npc_state: ResMut<State<NPCState>>,
    mut commands: Commands,
) {
    for (mut npc, mut transform) in npc_query.iter_mut() {

        if npc.state == NPCState::Stroll {
            // println!("{} is running...", npc.name);

            let direction: Vec3 = npc.direction;

            // TODO Approximation Louche
            if !close(transform.translation, direction)
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
                // println!("postChange: npc's state: {:#?}", npc.state);
                // ne sert a R atm
                
                commands.spawn()
                        .insert(RestTime {
                            // create the non-repeating rest timer
                            timer: Timer::new(Duration::from_secs(10), false),
                        });
            }
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

        /* example
            direction: [-1, 0.8, 0], Admiral pos: [-1.0008445, 0.79996014, 5]
            direction: [0.3, -1, 0], Olf pos: [0.29992545, -1.0007071, 5]
        */

    }
}

fn run_if_rest(
    mut npc_query: Query<&mut NPC>,
) -> ShouldRun
{
    for npc in npc_query.iter_mut() {
        if npc.state == NPCState::Rest {
            return ShouldRun::Yes;
        }
    }

    return ShouldRun::No;
}

fn rest(
    mut commands: Commands,
    time: Res<Time>,
    mut npc_query: Query<&mut NPC>,
    mut time_query: Query<(Entity, &mut RestTime)>,
) {
    for (entity, mut rest_timer) in time_query.iter_mut() {

        rest_timer.timer.tick(time.delta());

        for mut npc in npc_query.iter_mut() {

            if npc.state == NPCState::Rest {
                // println!("npc's state: {:#?}", npc.state);
    
                // flexing animation

                // println!(
                //     "{} is resting, rem time: {}",
                //     npc.name,
                //     rest_timer.timer.elapsed_secs()
                // );                      
    
                // TODO fix it.
                // When ONE timer is finished
                // all npc goes to work (walk)
                // the others timer still live
                if rest_timer.timer.finished() {
                    npc.state = NPCState::Stroll;
                    npc.direction = give_a_direction();
                    commands.entity(entity).despawn();
                }          
            }
        }
    }
}
            

#[derive(Component)]
struct RestTime {
    /// track when the npc should stop rest (non-repeating timer)
    timer: Timer,
}

/**
 * @param
 * position: of a entity
 * direction: the middle of the future zone, 
 *            is on the middle of the segment [a,c]
 * @return true
 * if the entity is on the square around the direction point
 */
fn close(
    position: Vec3,
    direction: Vec3,
) -> bool
{
    // direction.x == position.x &&
    // direction.y == position.y
    
    let a = 
        Vec3::new(
            direction.x-TILE_SIZE/2.0,
            direction.y+TILE_SIZE/2.0,
            direction.z
        );

    let c = 
        Vec3::new(
            direction.x+TILE_SIZE/2.0,
            direction.y-TILE_SIZE/2.0,
            direction.z
        );
    
    position.x >= a.x && position.x <= c.x &&
    position.y <= a.y && position.y >= c.y 
    
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

    /* shape ideas
     * (x, y) -> A
     * (x+1, y-1) -> C
     * (x+0.5, y-0.5) -> milieu
     */

    let direction: Vec3 = Vec3::new(x, y, 0.0);

    direction
}


/**
 * spawn a cuboid
 * which represent the zone around the destination of the npc
 */
fn spawn_destination(
    mut commands: Commands,
    mut npc_query: Query<&mut NPC>,
)
{
    for npc in npc_query.iter_mut() {
        commands
            .spawn()
            .insert(Collider::cuboid(TILE_SIZE/2.0, TILE_SIZE/2.0))
            .insert(Transform::from_xyz(npc.direction.x, npc.direction.y, 0.0));
    }
}

fn spawn_character(
    mut commands: Commands,
    fabien: Res<FabienSheet>
) {
    let position = Vec3::new(-0.2, 0.35, 6.0);

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
        Vec3::new(-0.2, 0.55, 6.0),
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
            state: NPCState::Stroll
        });

    commands
        .entity(olf)
        .insert(Name::new("NPC Olf"))
        .insert(
        NPC {
            name: "Olf".to_string(),
            speed: 3.0,
            direction: give_a_direction(),
            state: NPCState::Stroll
        });
}

fn show_ieud_grid(
    mut commands: Commands,
    fabien: Res<FabienSheet>
) {
    // TODO proper GRID

    let mut marks = Vec::new();

    for i in -10..10 {
        for j in -10..10 {
            let mark = spawn_fabien_sprite(
                &mut commands,
                &fabien,
                16,
                Color::rgb(0.9,0.9,0.9),
                Vec3::new(i as f32*0.1, j as f32*0.1, 4.0),
                Vec3::new(1.0,1.0,0.0)
            );
            let _name = 
                "Mark {a}.{b}".replace("{a}", &(i+10).to_string())
                              .replace("{b}", &(j+10).to_string());
            
            marks.push(mark);

            // commands
            //     .entity(mark)
            //     .insert(Name::new(name));
        }
    }

    commands
        .spawn()
        .insert(Name::new("Marks"))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .push_children(&marks);
}