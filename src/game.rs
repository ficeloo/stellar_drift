/***** GAME.RS *****/

use bevy::prelude::*;

#[derive(Resource)]
pub struct LevelState {
    pub level: u8,
}

pub fn is_entity_oob(transform: &mut Transform, half_width: f32, half_height: f32) {
    if transform.translation.x > half_width + 40.0 {
        transform.translation.x = -(half_width + 40.0);
    } else if transform.translation.x < -(half_width + 40.0) {
        transform.translation.x = half_width + 40.0;
    }
    if transform.translation.y > half_height + 40.0 {
        transform.translation.y = -(half_height + 40.0);
    } else if transform.translation.y < -(half_height + 40.0) {
        transform.translation.y = half_height + 40.0;
    }
}
