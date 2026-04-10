/***** MAIN.RS *****/
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod asteroid;
mod camera;
mod entity;
mod game;
mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })
        .add_systems(
            Startup,
            (
                camera::spawn_camera,
                player::spawn_player,
                asteroid::spawn_asteroid,
            ),
        )
        .add_systems(
            Update,
            (
                player::spawn_bullet,
                player::move_bullet,
                player::despawn_bullet,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (player::player_death, player::player_respawn).chain(),
        )
        .add_systems(Update, (player::move_player, asteroid::move_asteroid))
        .run()
}
