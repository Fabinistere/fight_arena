use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable};
use bevy_rapier2d::prelude::*;


use crate::{
    // collisions::{TesselatedCollider, TesselatedColliderConfig},
    combat::{
        Leader,
        stats::*,
        Team,
    },
    constants::{player::*, combat::team::TEAM_MC},
    FabienSheet,
    movement::*
};

pub struct PlayerPlugin;

#[derive(Component, Inspectable)]
pub struct Player;
#[derive(Component)]
pub struct PlayerSensor;

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
    let player = TextureAtlasSprite::new(4);
    // player.custom_size = Some(Vec2::splat(TILE_SIZE));

    // let basic_hitbox = asset_server.load("textures/character/basic_hitbox.png");
    // let morgan_hitbox: Handle<Image> = asset_server.load("textures/character/Morgan.png");

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: player,
            texture_atlas: fabien.0.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 6.),
                scale: Vec3::splat(PLAYER_SCALE),
                ..default()
            },
            ..default()
        }) 
        .insert(Name::new("Player"))
        .insert(Player)
        .insert(Leader)
        .insert(Team(TEAM_MC))
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert_bundle(MovementBundle {
            speed: Speed::default(),
            velocity: Velocity {
                linvel: Vect::ZERO,
                angvel: 0.0,
            }
        })
        // .insert(GravityScale(0.01))
        // .insert(Sleeping::disabled())
        // .insert(Ccd::enabled())
        .insert_bundle(CombatBundle {
            hp: HP {
                current_hp: PLAYER_HP,
                max_hp: PLAYER_HP
            },
            mana: MANA {
                current_mana: PLAYER_MANA,
                max_mana: PLAYER_MANA
            },
            initiative: Initiative (PLAYER_INITIATIVE),
            attack: Attack (PLAYER_ATTACK),
            attack_spe: AttackSpe(PLAYER_ATTACK_SPE),
            defense: Defense (PLAYER_DEFENSE),
            defense_spe: DefenseSpe (PLAYER_DEFENSE_SPE)
        })
        .with_children(|parent| {
            parent
                .spawn()
                .insert(Collider::cuboid(PLAYER_HITBOX_WIDTH, PLAYER_HITBOX_HEIGHT))
                .insert(Transform::from_xyz(0.0, PLAYER_HITBOX_Y_OFFSET, 0.0));

            // parent
            //     .spawn()
            //     .insert(Collider::segment(
            //         Vect::new(-PLAYER_HITBOX_WIDTH, 0.0),
            //         Vect::new(PLAYER_HITBOX_WIDTH, 0.0),
            //     ))
            //     .insert(Sensor)
            //     .insert(ActiveEvents::COLLISION_EVENTS)
            //     .insert(ActiveCollisionTypes::STATIC_STATIC)
            //     .insert(PlayerSensor);
        })
        // .with_children(|parent| {
        //     parent
        //         .spawn()
        //         .insert(TesselatedCollider {
        //             texture: morgan_hitbox.clone(),
        //             tesselator_config: TesselatedColliderConfig::default()
        //         })
        //         .insert(Transform::from_xyz(0.0, 0.0, 0.0));
        // })
        
        ;
}