use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable};
use bevy_rapier2d::prelude::*;


use crate::{
    // collisions::{TesselatedCollider, TesselatedColliderConfig},
    combat::stats::*,
    constants::player::*,
    FabienSheet,
    movement::*,
    spawn_fabien_sprite,

};

pub struct PlayerPlugin;

#[derive(Component, Inspectable)]
pub struct Player;

impl Plugin  for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_player)
            .add_system(player_movement.label("movement"))
            .add_system(camera_follow.after("movement"));

    }
}

fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>,
){
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
    
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&Speed, &mut Velocity), With<Player>>,
) {
    for (speed, mut rb_vel) in player_query.iter_mut() {

        let up = keyboard_input.pressed(KeyCode::Z);
        let down = keyboard_input.pressed(KeyCode::S);
        let left = keyboard_input.pressed(KeyCode::Q);
        let right = keyboard_input.pressed(KeyCode::D);

        let x_axis = -(right as i8) + left as i8;
        let y_axis = -(down as i8) + up as i8;

        let mut vel_x = x_axis as f32 * **speed;
        let mut vel_y = y_axis as f32 * **speed;

        if x_axis != 0 && y_axis != 0 {
            vel_x *= (std::f32::consts::PI / 4.0).cos();
            vel_y *= (std::f32::consts::PI / 4.0).cos();
        }

        rb_vel.linvel.x = vel_x;
        rb_vel.linvel.y = vel_y;
    }
}

fn spawn_player(
    mut commands: Commands,
    fabien: Res<FabienSheet>,
    // asset_server: Res<AssetServer>
)
{
    let player = spawn_fabien_sprite(
        &mut commands,
        &fabien,
        4,
        Color::rgb(0.9,0.9,0.9),
        Vec3::new(0.0, 0.0, 6.),
        Vec3::new(2.0, 2.0, 0.0)
    );

    // let basic_hitbox = asset_server.load("textures/character/basic_hitbox.png");
    // let morgan_hitbox = asset_server.load("textures/character/Morgan.png");

    commands
        .entity(player)
        .insert(Name::new("Player"))
        .insert(Player)
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert_bundle(MovementBundle {
            speed: Speed::default(),
            velocity: Velocity {
                linvel: Vec2::default(),
                angvel: 0.0,
            }
        })
        .insert_bundle(CombatBundle {
            hp: HP {
                current_hp: PLAYER_HP,
                max_hp: PLAYER_HP
            },
            mana: MANA {
                current_mana: PLAYER_MANA,
                max_mana: PLAYER_MANA
            },
            initiative: Initiative {
                initiative: PLAYER_INITIATIVE
            },
            attack: Attack {
                attack: PLAYER_ATTACK
            },
            attack_spe: AttackSpe {
                attack_spe: PLAYER_ATTACK_SPE
            },
            defense: Defense {
                defense: PLAYER_DEFENSE
            },
            defense_spe: DefenseSpe {
                defense_spe: PLAYER_DEFENSE_SPE
            }
        })
        // .insert(TesselatedCollider {
        //     texture: morgan_hitbox.clone(),
        //     tesselator_config: TesselatedColliderConfig {
        //         vertice_radius: 0.4,
        //         vertice_separation: 0.0,
        //         extrusion: 0.1,
        //     },
        // })
        ;
}