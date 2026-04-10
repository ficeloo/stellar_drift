/***** ASTEROID.RS *****/

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

use crate::entity::*;
use crate::game::*;

const A_S1_SPRITE_PATH: &str = "asteroid-1.png";
const A_S1_SPRITE_SIZE: f32 = 0.2;
const A_S1_SHAPE: f32 = 30.0;

const A_S2_SPRITE_PATH: &str = "asteroid-2.png";
const A_S2_SPRITE_SIZE: f32 = 0.4;
const A_S2_SHAPE: f32 = 60.0;

const A_S3_SPRITE_PATH: &str = "asteroid-3.png";
const A_S3_SPRITE_SIZE: f32 = 0.8;
const A_S3_SHAPE: f32 = 120.0;

const A_MAX_SPEED: f32 = 250.0;
const A_MAX_ROTATION_SPEED: f32 = 3.0;

const GROUP_PLAYER: Group = Group::GROUP_1;
const GROUP_ASTEROID: Group = Group::GROUP_2;
const GROUP_BULLET: Group = Group::GROUP_3;

#[derive(Component)]
pub struct Asteroid;

#[derive(Component, Clone, Copy)]
pub enum AsteroidSize {
    Large,
    Medium,
    Small,
}

#[derive(Bundle)]
pub struct AsteroidBundle {
    pub asteroid: Asteroid,
    pub size: AsteroidSize,
    pub health: Health,
    pub sprite: SpriteBundle,
    pub body: RigidBody,
    pub velocity: Velocity,
    pub shape: Collider,
    pub groups: CollisionGroups,
}

fn generate_random_position(width: f32, height: f32) -> Vec3 {
    let mut rng = rand::thread_rng();

    let x = rng.gen_range(-width..width);
    let y = rng.gen_range(-height..height);

    Vec3::new(x, y, 0.0)
}

fn generate_random_velocity(max_speed: f32) -> Vec2 {
    let mut rng = rand::thread_rng();

    let vel_x = rng.gen_range(-1.0..1.0);
    let vel_y = rng.gen_range(-1.0..1.0);
    let rand_speed = rng.gen_range(0.0..max_speed);
    Vec2::new(vel_x, vel_y).normalize() * rand_speed
}

fn generate_random_rotation() -> f32 {
    let mut rng = rand::thread_rng();

    rng.gen_range(-A_MAX_ROTATION_SPEED..A_MAX_ROTATION_SPEED)
}

pub fn move_asteroid(
    mut asteroid_query: Query<&mut Transform, With<Asteroid>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(window) = window_query.get_single() else {
        return;
    };
    let half_width = window.width() / 2.0;
    let half_height = window.height() / 2.0;

    for mut transform in asteroid_query.iter_mut() {
        is_entity_oob(&mut transform, half_width, half_height);
    }
}

fn create_asteroid(
    size: AsteroidSize,
    position: Vec3,
    asset_server: &AssetServer,
) -> AsteroidBundle {
    let (sprite_path, sprite_size, shape_size) = match size {
        AsteroidSize::Large => (A_S3_SPRITE_PATH, A_S3_SPRITE_SIZE, A_S3_SHAPE),
        AsteroidSize::Medium => (A_S2_SPRITE_PATH, A_S2_SPRITE_SIZE, A_S2_SHAPE),
        AsteroidSize::Small => (A_S1_SPRITE_PATH, A_S1_SPRITE_SIZE, A_S1_SHAPE),
    };

    let asteroid_asset = asset_server.load(sprite_path);

    let asteroid_sprite = SpriteBundle {
        texture: asteroid_asset,
        transform: Transform {
            translation: position, //
            scale: Vec3::splat(sprite_size),
            ..default()
        },
        ..default()
    };

    AsteroidBundle {
        asteroid: Asteroid,
        size,
        health: Health::new(1),
        sprite: asteroid_sprite,
        body: RigidBody::Dynamic,
        velocity: Velocity {
            linvel: generate_random_velocity(A_MAX_SPEED),
            angvel: generate_random_rotation(),
        },
        shape: Collider::ball(shape_size * sprite_size),
        groups: CollisionGroups::new(GROUP_ASTEROID, GROUP_PLAYER | GROUP_BULLET),
    }
}

pub fn spawn_asteroid(
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    // level_state: Res<LevelState>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(window) = window_query.get_single() else {
        return;
    };

    // Il faut que je rajoute le level_state afin d'augmenter le nombre d'asteroides
    for _ in 0..(0 + 3) {
        let position = generate_random_position(window.width() / 2.0, window.height() / 2.0);
        commands.spawn(create_asteroid(
            AsteroidSize::Large,
            position,
            &assets_server,
        ));
    }
}
