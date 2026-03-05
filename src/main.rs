/***** MAIN.RS *****/
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;

mod camera;
mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (camera::spawn_camera, player::spawn_player))
        .add_systems(Update, player::move_player)
        .run()
}
