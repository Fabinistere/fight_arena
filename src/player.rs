use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    // collisions::{TesselatedCollider, TesselatedColliderConfig},
    combat::{stats::*, InCombat, Karma, Leader, Team},
    constants::{
        character::{
            npc::dialog::MORGAN_DIALOG, player::*, CHAR_HITBOX_HEIGHT, CHAR_HITBOX_WIDTH,
            CHAR_HITBOX_Y_OFFSET,
        },
        combat::team::TEAM_MC,
    },
    movement::*,
    ui::dialog_system::Dialog,
    FabienSheet,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum PlayerSet {
    Movement,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player).add_systems((
            player_movement.in_set(PlayerSet::Movement),
            camera_follow.after(PlayerSet::Movement),
        ));
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerSensor;

fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&Speed, &mut Velocity), (With<Player>, Without<InCombat>)>,
) {
    // check if player_query is not empty
    if !player_query.is_empty() {
        let (speed, mut rb_vel) = player_query.single_mut();

        let up = keyboard_input.pressed(KeyCode::Z);
        let down = keyboard_input.pressed(KeyCode::S);
        let left = keyboard_input.pressed(KeyCode::Q);
        let right = keyboard_input.pressed(KeyCode::D);

        let x_axis = right as i8 - (left as i8);
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

fn spawn_player(mut commands: Commands, fabiens: Res<FabienSheet>) {
    commands
        .spawn((
            SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(PLAYER_STARTING_ANIM),
                texture_atlas: fabiens.0.clone(),
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 6.),
                    scale: Vec3::splat(PLAYER_SCALE),
                    ..default()
                },
                ..default()
            },
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            MovementBundle {
                speed: Speed::default(),
                velocity: Velocity {
                    linvel: Vect::ZERO,
                    angvel: 0.0,
                },
            },
            Name::new("Player"),
            Player,
            Dialog {
                current_node: Some(MORGAN_DIALOG.to_owned()),
            },
            Karma(10),
            // Combat
            Leader,
            Team(TEAM_MC),
            CombatBundle {
                hp: HP {
                    current_hp: PLAYER_HP,
                    max_hp: PLAYER_HP,
                },
                mana: MANA {
                    current_mana: PLAYER_MANA,
                    max_mana: PLAYER_MANA,
                },
                initiative: Initiative(PLAYER_INITIATIVE),
                attack: Attack(PLAYER_ATTACK),
                attack_spe: AttackSpe(PLAYER_ATTACK_SPE),
                defense: Defense(PLAYER_DEFENSE),
                defense_spe: DefenseSpe(PLAYER_DEFENSE_SPE),
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Collider::cuboid(CHAR_HITBOX_WIDTH, CHAR_HITBOX_HEIGHT),
                Transform::from_xyz(0.0, CHAR_HITBOX_Y_OFFSET, 0.0),
                CharacterHitbox,
            ));

            // parent
            //     .spawn()
            //     .insert(Collider::segment(
            //         Vect::new(-CHAR_HITBOX_WIDTH, 0.0),
            //         Vect::new(CHAR_HITBOX_WIDTH, 0.0),
            //     ))
            //     .insert(Sensor)
            //     .insert(ActiveEvents::COLLISION_EVENTS)
            //     .insert(ActiveCollisionTypes::STATIC_STATIC)
            //     .insert(PlayerSensor);
        });
}
