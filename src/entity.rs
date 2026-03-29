/***** ENTITY.RS *****/

use bevy::prelude::*;

#[derive(Component)]
pub struct Movement {
    pub velocity: Vec3,
    pub rotation_speed: f32,
}

impl Movement {
    pub fn new(velocity: Vec3, rotation_speed: f32) -> Self {
        Self {
            velocity,
            rotation_speed,
        }
    }
}

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
