/***** ASTEROID.RS *****/

use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

use crate::player::{Health, Movement};

const A_S1_SPRITE_PATH: &str = "asteroid-1.png";
const A_S1_SPRITE_SIZE: f32 = 0.3;
const A_MAX_SPEED: f32 = 150.0;

#[derive(Component)]
pub struct Asteroid;

#[derive(Bundle)]
pub struct AsteroidBundle {
    pub asteroid: Asteroid,
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

pub fn spawn_asteroid_small(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(window) = window_query.get_single() else {
        return;
    };

    let asteroid_asset = asset_server.load(A_S1_SPRITE_PATH);

    let asteroid_sprite = SpriteBundle {
        texture: asteroid_asset,
        transform: Transform {
            translation: generate_random_position(window.width() / 2.0, window.height() / 2.0),
            scale: Vec3::splat(A_S1_SPRITE_SIZE),
            ..default()
        },
        ..default()
    };

    let asteroid_bundle = AsteroidBundle {
        asteroid: Asteroid,
        health: Health::new(1),
        movement: Movement::new(generate_random_velocity(A_MAX_SPEED)),
        sprite: asteroid_sprite,
    };

    commands.spawn(asteroid_bundle);
}
