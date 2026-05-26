/***** GAME.RS *****/

use bevy::prelude::*;
use bevy_rapier2d::plugin::RapierConfiguration;

use crate::asteroid::Asteroid;
use crate::entity::Health;
use crate::states::GameState;

#[derive(Resource)]
pub struct LevelState {
    pub level: u8,
    pub health: Health,
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

pub fn level_complete(
    asteroid_query: Query<&Asteroid>,
    mut next_state: ResMut<NextState<GameState>>,
    mut level_state: ResMut<LevelState>,
) {
    if level_state.health.current == 0 {
        next_state.set(GameState::GameOver);
    }

    if asteroid_query.is_empty() {
        level_state.level += 1;
        next_state.set(GameState::LevelTransition);
    }
}

pub fn set_physics_true(mut rapier_conf: ResMut<RapierConfiguration>) {
    rapier_conf.physics_pipeline_active = true;
}

pub fn set_physics_false(mut rapier_conf: ResMut<RapierConfiguration>) {
    rapier_conf.physics_pipeline_active = false;
}
