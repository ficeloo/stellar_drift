/***** AUDIO.RS *****/

use bevy::audio::*;
use bevy::prelude::*;

#[derive(Resource)]
pub struct AudioAssets {
    pub laser: Handle<AudioSource>,
    pub impact: Handle<AudioSource>,
    pub engine: Handle<AudioSource>,
}

#[derive(Component)]
pub struct EngineSound;

pub fn load_audio(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(AudioAssets {
        laser: asset_server.load("bullet.mp3"),
        impact: asset_server.load("asteroid-impact.mp3"),
        engine: asset_server.load("ship-flying.wav"),
    });
}
