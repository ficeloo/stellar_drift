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
        next_state.set(GameState::LevelTransition);
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
