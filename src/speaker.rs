use std::collections::HashMap;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

lazy_static! {
static ref SOUND_POOL: HashMap<&'static str, &'static str> = {
        HashMap::from([
            ("primary", "square440.wav"),
            ])
    };
}

#[derive(Resource)]
struct Square {
    source: Handle<AudioSource>,
    gain: u8,
}

pub fn install_speaker(mut commands: Commands, server: Res<AssetServer>) {
    if let Some(primary) = SOUND_POOL.get("primary") {
        let audio: Handle<AudioSource> = server.load(*primary);
        commands.insert_resource(Square {
            source: audio,
            gain: 1,
        });
    }
}
