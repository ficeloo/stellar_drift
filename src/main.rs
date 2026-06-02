/***** MAIN.RS *****/
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::asteroid::AsteroidHitEvent;
use crate::entity::Health;
use crate::game::LevelState;
use crate::states::GameState;

mod asteroid;
mod camera;
mod entity;
mod game;
mod hud;
mod player;
mod states;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })
        .insert_resource(LevelState {
            level: 0,
            health: Health { current: 3, max: 3 },
        })
        .add_state::<GameState>()
        .add_event::<AsteroidHitEvent>();

    app.add_systems(Startup, camera::spawn_camera);

    app.add_systems(Update, states::menu.run_if(in_state(GameState::MainMenu)));
    app.add_systems(OnEnter(GameState::MainMenu), entity::remove_all_entities);

    /*** |HUD| ***/
    app.add_systems(OnEnter(GameState::MainMenu), hud::spawn_menu_hud);
    app.add_systems(OnExit(GameState::MainMenu), hud::despawn_hud);

    app.add_systems(
        OnEnter(GameState::LevelTransition),
        (hud::update_hud, hud::despawn_hud),
    );

    app.add_systems(OnEnter(GameState::Playing), hud::spawn_playing_hud);
    app.add_systems(Update, hud::update_hud.run_if(in_state(GameState::Playing)));

    app.add_systems(OnEnter(GameState::Paused), hud::spawn_pause_hud);
    app.add_systems(OnExit(GameState::Paused), hud::despawn_hud);

    app.add_systems(OnEnter(GameState::MainMenu), hud::despawn_hud);
    app.add_systems(OnEnter(GameState::GameOver), hud::despawn_hud);

    app.add_systems(OnEnter(GameState::GameOver), hud::spawn_game_over_hud);
    app.add_systems(OnExit(GameState::GameOver), hud::despawn_hud);

    app.add_systems(
        OnEnter(GameState::FirstLevel),
        (player::spawn_player, asteroid::spawn_asteroid),
    );

    app.add_systems(
        OnEnter(GameState::LevelTransition),
        asteroid::spawn_asteroid,
    );

    app.add_systems(
        Update,
        (
            states::loading.run_if(in_state(GameState::FirstLevel)),
            states::loading.run_if(in_state(GameState::LevelTransition)),
        ),
    );

    app.add_systems(Update, states::paused.run_if(in_state(GameState::Paused)));
    app.add_systems(Update, states::playing.run_if(in_state(GameState::Playing)));
    app.add_systems(OnEnter(GameState::Paused), game::set_physics_false);
    app.add_systems(OnEnter(GameState::Playing), game::set_physics_true);

    app.add_systems(
        Update,
        (
            player::spawn_bullet,
            player::move_bullet,
            player::despawn_bullet,
        )
            .chain()
            .run_if(in_state(GameState::Playing)),
    );

    app.add_systems(
        Update,
        (player::player_death, player::player_respawn)
            .chain()
            .run_if(in_state(GameState::Playing)),
    );
    app.add_systems(
        Update,
        (player::move_player, asteroid::move_asteroid).run_if(in_state(GameState::Playing)),
    );
    app.add_systems(
        Update,
        (asteroid::handle_asteroid_hit, game::level_complete)
            .chain()
            .run_if(in_state(GameState::Playing)),
    );

    app.add_systems(OnEnter(GameState::GameOver), entity::remove_all_entities);
    app.add_systems(
        Update,
        states::game_over.run_if(in_state(GameState::GameOver)),
    );

    app.run();
}
