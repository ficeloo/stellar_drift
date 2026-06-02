use crate::game::LevelState;
use bevy::prelude::*;

// --- MARQUEURS ---

#[derive(Component)]
pub struct HudScreen;

#[derive(Component)]
pub struct HealthText;

#[derive(Component)]
pub struct LevelText;

// --- MENU PRINCIPAL ---

pub fn spawn_menu_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: Color::rgb(0.05, 0.05, 0.05).into(),
                ..default()
            },
            HudScreen,
        ))
        .with_children(|parent| {
            // Titre
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "STELLAR DRIFT",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 60.0,
                        color: Color::WHITE,
                    },
                ),
                style: Style {
                    margin: UiRect::bottom(Val::Px(30.0)),
                    ..default()
                },
                ..default()
            });
            // Sous-titre
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Press ENTER to play",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 25.0,
                        color: Color::GRAY,
                    },
                ),
                ..default()
            });
        });
}

// --- HUD IN-GAME ---

pub fn spawn_playing_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    level_state: Res<LevelState>,
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::SpaceBetween,
                    padding: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                ..default()
            },
            HudScreen,
        ))
        .with_children(|parent| {
            // Vies (haut gauche)
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        format!("HEALTH: {}", level_state.health.current),
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 30.0,
                            color: Color::rgb(0.9, 0.2, 0.2),
                        },
                    ),
                    ..default()
                },
                HealthText,
            ));

            // Niveau (haut droite)
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        format!("LEVEL: {}", level_state.level),
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 30.0,
                            color: Color::rgb(0.2, 0.8, 0.2),
                        },
                    ),
                    ..default()
                },
                LevelText,
            ));
        });
}

// --- GAME OVER ---

pub fn spawn_game_over_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: Color::rgba(0.1, 0.0, 0.0, 0.85).into(),
                ..default()
            },
            HudScreen,
        ))
        .with_children(|parent| {
            // "GAME OVER"
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "GAME OVER",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 70.0,
                        color: Color::RED,
                    },
                ),
                style: Style {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
                ..default()
            });
            // Instructions
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "ENTER: Try again |  ECHAP: Menu",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 25.0,
                        color: Color::WHITE,
                    },
                ),
                ..default()
            });
        });
}

// --- PAUSE ---

pub fn spawn_pause_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: Color::rgba(0.0, 0.0, 0.0, 0.6).into(),
                ..default()
            },
            HudScreen,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "PAUSE",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 60.0,
                        color: Color::WHITE,
                    },
                ),
                style: Style {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
                ..default()
            });
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "ENTER: Resume  |  ECHAP: Menu",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 25.0,
                        color: Color::GRAY,
                    },
                ),
                ..default()
            });
        });
}

// --- MISE À JOUR ---

pub fn update_hud(
    level_state: Res<LevelState>,
    mut health_query: Query<&mut Text, (With<HealthText>, Without<LevelText>)>,
    mut level_query: Query<&mut Text, (With<LevelText>, Without<HealthText>)>,
) {
    if level_state.is_changed() {
        if let Ok(mut text) = health_query.get_single_mut() {
            text.sections[0].value = format!("HEALTH: {}", level_state.health.current);
        }
        if let Ok(mut text) = level_query.get_single_mut() {
            text.sections[0].value = format!("LEVEL: {}", level_state.level);
        }
    }
}

// --- NETTOYAGE ---

pub fn despawn_hud(mut commands: Commands, query: Query<Entity, With<HudScreen>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
