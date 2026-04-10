/***** PLAYER.RS *****/

use crate::entity::*;
use crate::game::*;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;

const P_SPRITE_SIZE: f32 = 0.3;
const P_SPRITE_PATH: &str = "stellar_drifter.png";
const P_SPEED: f32 = 10.0;
const P_ROT_SPEED: f32 = 0.4;
const P_SHAPE: f32 = 200.0;
const LIN_DAMP: f32 = 1.2;
const ANG_DAMP: f32 = 9.0;

const B_SPRITE_PATH: &str = "bullet.png";
const B_SPRITE_SIZE: f32 = 0.3;
const B_SPEED: f32 = 1000.0;
const B_SHAPE: f32 = 10.0;

const GROUP_PLAYER: Group = Group::GROUP_1;
const GROUP_ASTEROID: Group = Group::GROUP_2;
const GROUP_BULLET: Group = Group::GROUP_3;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct PlayerTimer {
    pub noclip: GameTimer,
    pub dispawn: GameTimer,
}

#[derive(Bundle)]
pub struct BulletBundle {
    pub bullet: Bullet,
    pub sprite: SpriteBundle,
    pub timer: GameTimer,
    pub body: RigidBody,
    pub velocity: Velocity,
    pub shape: Collider,
    pub sensor: Sensor,
    pub events: ActiveEvents,
    pub groups: CollisionGroups,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub health: Health,
    pub sprite: SpriteBundle,
    pub timer: PlayerTimer,
    pub body: RigidBody,
    pub velocity: Velocity,
    pub damping: Damping,
    pub shape: Collider,
    pub events: ActiveEvents,
    pub groups: CollisionGroups,
}

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&PlayerTimer, &mut Transform, &mut Velocity), With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(window) = window_query.get_single() else {
        return;
    };

    let half_width = window.width() / 2.0;
    let half_height = window.height() / 2.0;

    if let Ok((timers, mut transform, mut velocity)) = player_query.get_single_mut() {
        if !timers.dispawn.timer.finished() {
            return;
        }
        if keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]) {
            velocity.angvel += P_ROT_SPEED; //transform.rotate_z(P_ROT_SPEED * time.delta_seconds());
        }
        if keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]) {
            velocity.angvel -= P_ROT_SPEED;
            // transform.rotate_z(-P_ROT_SPEED * time.delta_seconds());
        }
        let direction = transform.up().truncate();
        if keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]) {
            velocity.linvel += direction * P_SPEED;
        }
        if keyboard_input.any_pressed([KeyCode::S, KeyCode::Down]) {
            velocity.linvel -= ((direction * P_SPEED) / 4.0).clamp_length(0.0, 5.0);
        }

        is_entity_oob(&mut transform, half_width, half_height);
    }
}

pub fn move_bullet(
    mut bullet_query: Query<&mut Transform, With<Bullet>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(window) = window_query.get_single() else {
        return;
    };

    let half_width = window.width() / 2.0;
    let half_height = window.height() / 2.0;

    for mut transform in bullet_query.iter_mut() {
        is_entity_oob(&mut transform, half_width, half_height);
    }
}

pub fn despawn_bullet(
    mut commands: Commands,
    mut bullet_query: Query<(Entity, &mut GameTimer), With<Bullet>>,
    time: Res<Time>,
    mut collide: EventReader<CollisionEvent>,
) {
    for (entity, mut timer) in bullet_query.iter_mut() {
        timer.timer.tick(time.delta());
        if timer.timer.finished() {
            commands.entity(entity).despawn();
        }
    }

    // ACTUELLEMENT DISPAWN AU CONTACT DU JOUEUR, IL FAUT CREER UN COLLISION GROUP
    for colliding in collide.read() {
        if let CollisionEvent::Started(e1, e2, _) = *colliding {
            for entity in [e1, e2] {
                if bullet_query.contains(entity) {
                    if let Some(mut cmd) = commands.get_entity(entity) {
                        cmd.despawn();
                    }
                }
            }
        }
    }
}

pub fn player_respawn(
    mut player_query: Query<(&mut PlayerTimer, &mut Visibility, &mut CollisionGroups)>,
    time: Res<Time>,
) {
    if let Ok((mut timers, mut visible, mut groups)) = player_query.get_single_mut() {
        let delta = time.delta();

        timers.dispawn.timer.tick(delta);
        timers.noclip.timer.tick(delta);

        if timers.dispawn.timer.just_finished() {
            *visible = Visibility::Visible;
        }

        if timers.noclip.timer.just_finished() {
            groups.filters = Group::GROUP_2;
        }
    }
}

pub fn player_death(
    mut player_query: Query<
        (
            &mut PlayerTimer,
            &mut Velocity,
            &mut Transform,
            &mut CollisionGroups,
            &mut Visibility,
        ),
        With<Player>,
    >,
    mut collide: EventReader<CollisionEvent>,
) {
    for colliding in collide.read() {
        if let CollisionEvent::Started(e1, e2, _) = *colliding {
            for entity in [e1, e2] {
                if let Ok((mut timers, mut velocity, mut transform, mut groups, mut visibility)) =
                    player_query.get_mut(entity)
                {
                    if timers.noclip.timer.finished() {
                        *visibility = Visibility::Hidden;
                        transform.translation = Vec3::ZERO;
                        transform.rotation = Quat::IDENTITY;
                        velocity.angvel = 0.0;
                        velocity.linvel = Vec2::ZERO;
                        groups.filters = Group::NONE;
                        timers.noclip.timer.reset();
                        timers.dispawn.timer.reset();
                    }
                }
            }
        }
    }
}

pub fn spawn_bullet(
    mut commands: Commands,
    player_query: Query<&mut Transform, With<Player>>,
    assets_server: Res<AssetServer>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if !keyboard_input.just_pressed(KeyCode::Space) {
        return;
    }

    let Ok(transform) = player_query.get_single() else {
        return;
    };

    let bullet_asset = assets_server.load(B_SPRITE_PATH);

    let bullet_sprite = SpriteBundle {
        texture: bullet_asset,
        transform: Transform {
            translation: transform.translation,
            rotation: transform.rotation,
            scale: Vec3::splat(B_SPRITE_SIZE),
        },
        ..default()
    };

    let bullet_bundle = BulletBundle {
        bullet: Bullet,
        sprite: bullet_sprite,
        timer: GameTimer::new(1.5, TimerMode::Once),
        body: RigidBody::KinematicVelocityBased,
        velocity: Velocity::linear(B_SPEED * transform.up().truncate()),
        shape: Collider::ball(B_SPRITE_SIZE * B_SHAPE),
        sensor: Sensor,
        events: ActiveEvents::COLLISION_EVENTS,
        groups: CollisionGroups::new(GROUP_BULLET, GROUP_ASTEROID),
    };

    commands.spawn(bullet_bundle);
}

pub fn spawn_player(mut commands: Commands, assets_server: Res<AssetServer>) {
    let ship_asset = assets_server.load(P_SPRITE_PATH);

    let ship_sprite = SpriteBundle {
        texture: ship_asset,
        transform: Transform::from_scale(Vec3::splat(P_SPRITE_SIZE)),
        ..default()
    };

    let player_bundle = PlayerBundle {
        player: Player,
        health: Health::new(3),
        sprite: ship_sprite,
        timer: PlayerTimer {
            noclip: GameTimer::new(2.0, TimerMode::Once),
            dispawn: GameTimer::new(3.0, TimerMode::Once),
        },
        body: RigidBody::Dynamic,
        velocity: Velocity::default(),
        damping: Damping {
            linear_damping: LIN_DAMP,
            angular_damping: ANG_DAMP,
        },
        shape: Collider::ball(P_SPRITE_SIZE * P_SHAPE),
        events: ActiveEvents::COLLISION_EVENTS,
        groups: CollisionGroups::new(GROUP_PLAYER, GROUP_ASTEROID),
    };
    commands.spawn(player_bundle);
}
