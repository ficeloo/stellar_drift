/***** PLAYER.RS *****/

use crate::entity::*;
use crate::game::*;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;

const P_SPRITE_SIZE: f32 = 0.3;
const P_SPRITE_PATH: &str = "stellar_drifter.png";
const P_SPEED: f32 = 10.0;
const P_ROT_SPEED: f32 = 0.4;
const LIN_DAMP: f32 = 1.2;
const ANG_DAMP: f32 = 9.0;

const B_SPRITE_PATH: &str = "bullet.png";
const B_SPRITE_SIZE: f32 = 0.3;
const B_SPEED: f32 = 1000.0;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Bullet;

#[derive(Bundle)]
pub struct BulletBundle {
    pub bullet: Bullet,
    pub movement: Movement,
    pub sprite: SpriteBundle,
    pub timer: LifeTime,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub health: Health,
    pub sprite: SpriteBundle,
    pub body: RigidBody,
    pub velocity: Velocity,
    pub damping: Damping,
    pub shape: Collider,
}

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Velocity), With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(window) = window_query.get_single() else {
        return;
    };

    let half_width = window.width() / 2.0;
    let half_height = window.height() / 2.0;

    if let Ok((mut transform, mut velocity)) = player_query.get_single_mut() {
        if keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]) {
            velocity.angvel += P_ROT_SPEED; //transform.rotate_z(P_ROT_SPEED * time.delta_seconds());
        }
        if keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]) {
            velocity.angvel -= P_ROT_SPEED;
            // transform.rotate_z(-P_ROT_SPEED * time.delta_seconds());
        }
        let direction = transform.up().truncate();
        if keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]) {
            velocity.linvel += direction * P_SPEED;
        }
        if keyboard_input.any_pressed([KeyCode::S, KeyCode::Down]) {
            velocity.linvel -= ((direction * P_SPEED) / 4.0).clamp_length(0.0, 5.0);
        }

        is_entity_oob(&mut transform, half_width, half_height);
    }
}

pub fn move_bullet(
    mut bullet_query: Query<(&mut Transform, &mut Movement), With<Bullet>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    delta_time: Res<Time>,
) {
    let Ok(window) = window_query.get_single() else {
        return;
    };

    let delta_time = delta_time.delta_seconds();
    let half_width = window.width() / 2.0;
    let half_height = window.height() / 2.0;

    for (mut transform, movement) in bullet_query.iter_mut() {
        transform.translation += movement.velocity * delta_time;

        is_entity_oob(&mut transform, half_width, half_height);
    }
}

pub fn despawn_bullet(
    mut commands: Commands,
    mut bullet_query: Query<(Entity, &mut LifeTime), With<Bullet>>,
    time: Res<Time>,
) {
    for (entity, mut timer) in bullet_query.iter_mut() {
        timer.timer.tick(time.delta());
        if timer.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn spawn_bullet(
    mut commands: Commands,
    player_query: Query<&mut Transform, With<Player>>,
    assets_server: Res<AssetServer>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if !keyboard_input.just_pressed(KeyCode::Space) {
        return;
    }

    let Ok(transform) = player_query.get_single() else {
        return;
    };

    let bullet_asset = assets_server.load(B_SPRITE_PATH);

    let bullet_sprite = SpriteBundle {
        texture: bullet_asset,
        transform: Transform {
            translation: transform.translation,
            rotation: transform.rotation,
            scale: Vec3::splat(B_SPRITE_SIZE),
        },
        ..default()
    };

    let bullet_bundle = BulletBundle {
        bullet: Bullet,
        movement: Movement::new(B_SPEED * transform.up(), 0.0),
        sprite: bullet_sprite,
        timer: LifeTime::new(1.5, TimerMode::Once),
    };

    commands.spawn(bullet_bundle);
}

pub fn spawn_player(mut commands: Commands, assets_server: Res<AssetServer>) {
    let ship_asset = assets_server.load(P_SPRITE_PATH);

    let ship_sprite = SpriteBundle {
        texture: ship_asset,
        transform: Transform::from_scale(Vec3::splat(P_SPRITE_SIZE)),
        ..default()
    };

    let player_bundle = PlayerBundle {
        player: Player,
        health: Health::new(3),
        sprite: ship_sprite,
        body: RigidBody::Dynamic,
        velocity: Velocity::default(),
        damping: Damping {
            linear_damping: LIN_DAMP,
            angular_damping: ANG_DAMP,
        },
        shape: Collider::ball(P_SPRITE_SIZE * 10.0),
    };
    commands.spawn(player_bundle);
}
