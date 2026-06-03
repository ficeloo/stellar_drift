/***** ENTITY.RS *****/

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub const GROUP_PLAYER: Group = Group::GROUP_1;
pub const GROUP_ASTEROID: Group = Group::GROUP_2;
pub const GROUP_BULLET: Group = Group::GROUP_3;

#[derive(Component)]
pub struct Health {
    pub current: u8,
    pub max: u8,
}

impl Health {
    pub fn new(amount: u8) -> Self {
        Self {
            current: amount,
            max: amount,
        }
    }
}

#[derive(Component)]
pub struct Despawning;

#[derive(Component)]
pub struct GameTimer {
    pub timer: Timer,
}

impl GameTimer {
    pub fn new(duration: f32, mode: TimerMode) -> Self {
        Self {
            timer: Timer::from_seconds(duration, mode),
        }
    }
}

pub fn apply_despawn(mut commands: Commands, query: Query<Entity, With<Despawning>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn remove_all_entities(mut commands: Commands, entity_query: Query<Entity, With<RigidBody>>) {
    for entity in &entity_query {
        commands.entity(entity).despawn_recursive();
    }
}
