/***** STATES.RS *****/

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::game::LevelState;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    Playing,
    Paused,
    FirstLevel,
    LevelTransition,
    GameOver,
}

pub fn menu(
    keyboard_input: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut level_state: ResMut<LevelState>,
) {
    if keyboard_input.just_pressed(KeyCode::Return) {
        level_state.level = 0;
        level_state.health.current = level_state.health.max;
        next_state.set(GameState::FirstLevel);
    }
}

pub fn loading(
    entity_query: Query<(), With<RigidBody>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if !entity_query.is_empty() {
        next_state.set(GameState::Playing);
    }
}

pub fn pause(
    keyboard_input: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    curr_state: Res<State<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match curr_state.get() {
            GameState::Playing => {
                next_state.set(GameState::Paused);
            }
            GameState::Paused => {
                next_state.set(GameState::Playing);
            }
            _ => {}
        }
    }
}

pub fn game_over(
    keyboard_input: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut level_state: ResMut<LevelState>,
) {
    if keyboard_input.just_pressed(KeyCode::Return) {
        level_state.level = 0;
        level_state.health.current = level_state.health.max;
        next_state.set(GameState::FirstLevel);
    } else if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::MainMenu)
    }
}
