/***** MAIN.RS *****/
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;

mod asteroid;
mod camera;
mod entity;
mod game;
mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
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
                (player::spawn_bullet, player::move_bullet).chain(),
                player::move_player,
                asteroid::move_asteroid,
            ),
        )
        .run()
}
