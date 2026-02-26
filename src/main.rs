/***** MAIN.RS *****/

use bevy::prelude::*;

mod camera;
mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (camera::spawn_camera, player::spawn_player))
        .run()
}
