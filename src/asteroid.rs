/***** ASTEROID.RS *****/

use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

use crate::game::LevelState;
use crate::player::{Health, Movement};

const A_S1_SPRITE_PATH: &str = "asteroid-1.png";
const A_S1_SPRITE_SIZE: f32 = 0.2;

const A_S2_SPRITE_PATH: &str = "asteroid-2.png";
const A_S2_SPRITE_SIZE: f32 = 0.4;

const A_S3_SPRITE_PATH: &str = "asteroid-3.png";
const A_S3_SPRITE_SIZE: f32 = 0.6;

const A_MAX_SPEED: f32 = 150.0;

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
    pub movement: Movement,
    pub sprite: SpriteBundle,
}

fn generate_random_position(width: f32, height: f32) -> Vec3 {
    let mut rng = rand::thread_rng();

    let x = rng.gen_range(-width..width);
    let y = rng.gen_range(-height..height);

    Vec3::new(x, y, 0.0)
}

fn generate_random_velocity(max_speed: f32) -> Vec3 {
    let mut rng = rand::thread_rng();

    let vel_x = rng.gen_range(-1.0..1.0);
    let vel_y = rng.gen_range(-1.0..1.0);
    let rand_speed = rng.gen_range(0.0..max_speed);
    Vec3::new(vel_x, vel_y, 0.0).normalize() * rand_speed
}

fn create_asteroid(
    size: AsteroidSize,
    window: &Window,
    asset_server: &AssetServer,
) -> AsteroidBundle {
    let (sprite_path, sprite_size) = match size {
        AsteroidSize::Large => (A_S3_SPRITE_PATH, A_S3_SPRITE_SIZE),
        AsteroidSize::Medium => (A_S2_SPRITE_PATH, A_S2_SPRITE_SIZE),
        AsteroidSize::Small => (A_S1_SPRITE_PATH, A_S1_SPRITE_SIZE),
    };

    let asteroid_asset = asset_server.load(sprite_path);

    let asteroid_sprite = SpriteBundle {
        texture: asteroid_asset,
        transform: Transform {
            translation: generate_random_position(window.width() / 2.0, window.height() / 2.0),
            scale: Vec3::splat(sprite_size),
            ..default()
        },
        ..default()
    };

    AsteroidBundle {
        asteroid: Asteroid,
        size,
        health: Health::new(1),
        movement: Movement::new(generate_random_velocity(A_MAX_SPEED)),
        sprite: asteroid_sprite,
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
        commands.spawn(create_asteroid(AsteroidSize::Large, window, &assets_server));
    }
}
