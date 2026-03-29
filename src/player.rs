/***** PLAYER.RS *****/

use crate::entity::*;
use crate::game::*;
use bevy::{prelude::*, window::PrimaryWindow};

const P_SPRITE_SIZE: f32 = 0.3;
const P_SPRITE_PATH: &str = "stellar_drifter.png";
const P_SPEED: f32 = 1750.0;
const P_ROT_SPEED: f32 = 5.0;

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
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub health: Health,
    pub movement: Movement,
    pub sprite: SpriteBundle,
}

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Movement), With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    delta_time: Res<Time>,
) {
    let Ok(window) = window_query.get_single() else {
        return;
    };

    let delta_time = delta_time.delta_seconds();
    let half_width = window.width() / 2.0;
    let half_height = window.height() / 2.0;

    if let Ok((mut transform, mut movement)) = player_query.get_single_mut() {
        let direction = transform.up();

        if keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]) {
            movement.velocity += direction * P_SPEED * delta_time;
        }
        if keyboard_input.any_pressed([KeyCode::S, KeyCode::Down]) {
            movement.velocity -= ((direction * P_SPEED * delta_time) / 4.0).clamp_length(0.0, 50.0);
        }
        if keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]) {
            transform.rotate_z(P_ROT_SPEED * delta_time);
        }
        if keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]) {
            transform.rotate_z(-P_ROT_SPEED * delta_time);
        }
        movement.velocity = (movement.velocity * 0.98).clamp_length(0.0, 15000.0);

        // Principe d'inertie dans le vide
        transform.translation += movement.velocity * delta_time;

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

pub fn spawn_bullet(
    mut commands: Commands,
    player_query: Query<(&mut Transform, &mut Movement), With<Player>>,
    assets_server: Res<AssetServer>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if !keyboard_input.just_pressed(KeyCode::Space) {
        return;
    }

    let Ok((transform, movement)) = player_query.get_single() else {
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
        movement: Movement::new(Vec3::ZERO, 0.0),
        sprite: ship_sprite,
    };
    commands.spawn(player_bundle);
}
