/***** PLAYER.RS *****/

use bevy::prelude::*;

const P_SPRITE_SIZE: f32 = 0.3;
const P_SPRITE_PATH: &str = "stellar_drifter.png";
const P_SPEED: f32 = 600.0;
const P_ROT_SPEED: f32 = 5.0;

#[derive(Component)]
pub struct Movement {
    pub velocity: Vec3,
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
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Movement), With<Player>>,
    delta_time: Res<Time>,
) {
    let delta_time = delta_time.delta_seconds();


    for (mut transform, mut movement) in player_query.iter_mut() {
        let direction = transform.up();

        if keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]) {
            movement.velocity += direction * P_SPEED * delta_time;
        }
        if keyboard_input.any_pressed([KeyCode::S, KeyCode::Down]) {
            movement.velocity -= ((direction * P_SPEED * delta_time) / 4.0).clamp_length(0.0, 50.0);
        }
        if keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]) {
            transform.rotate_z(P_ROT_SPEED * delta_time);
        }
        if keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]) {
            transform.rotate_z(-P_ROT_SPEED * delta_time);
        }
        movement.velocity = (movement.velocity * 0.98).clamp_length(0.0, 500.0);

        // Principe d'inertie dans le vide
        transform.translation += movement.velocity * delta_time;
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
        movement: Movement { velocity: Vec3::ZERO },
        sprite: ship_sprite,
    };
    commands.spawn(player_bundle);
}
