/***** PLAYER.RS *****/

use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub fn spawn_player(mut commands: Commands, assets_server: Res<AssetServer>) {
    let ship_asset = assets_server.load("stellar_drifter.png");
    let ship_sprite = SpriteBundle {
        texture: ship_asset,
        ..default()
    };

    commands.spawn((ship_sprite, Player));
}
