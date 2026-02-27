/***** PLAYER.RS *****/

use bevy::prelude::*;

const P_SPRITE_SIZE: f32 = 0.3;
const P_SPRITE_PATH: &str = "stellar_drifter.png";
const P_SPEED: f32 = 15.0;
const P_ROT_SPEED: f32 = 5.0;

#[derive(Component)]
pub struct Movement {
    pub velocity: f32,
}

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub movement: Movement,
    pub sprite: SpriteBundle,
}

pub fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Movement), With<Player>>,
    delta_time: Res<Time>,
) {
    let delta_time = delta_time.delta_seconds();

    for (mut transform, mut movement) in player_query.iter_mut() {
        let direction = transform.up();

        // Principe d'inertie dans le vide
        transform.translation += direction * movement.velocity * delta_time;

        if keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]) {
            transform.translation += direction * P_SPEED * delta_time;
            movement.velocity += P_SPEED;
        }
        if keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]) {
            transform.translation -= direction * (P_SPEED / 20.0) * delta_time;
            movement.velocity -= P_SPEED / 20.0;
        }
        if keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
            transform.rotate_z(P_ROT_SPEED * delta_time);
        }
        if keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
            transform.rotate_z(-P_ROT_SPEED * delta_time);
        }
        movement.velocity = (movement.velocity - 5.0).clamp(0.0, 1000.0);
    }
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
        movement: Movement { velocity: 0.0 },
        sprite: ship_sprite,
    };
    commands.spawn(player_bundle);
}
