pub mod music;

use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<ConfigHandle<AudioConfig>>();

    app.add_plugins(music::plugin);
}

#[derive(Asset, Reflect, Serialize, Deserialize)]
pub struct AudioConfig {
    /// The precise beats-per-minute of the music.
    pub bpm: f64,
    /// The position (in seconds) of the zeroth beat.
    pub zeroth_beat: f64,
}

impl Config for AudioConfig {
    const PATH: &'static str = "config/audio.ron";
    const EXTENSION: &'static str = "audio.ron";
}
