pub mod beat;

use std::time::Duration;

use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::game::music::beat::BeatTimer;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<ConfigHandle<MusicConfig>>();

    app.add_plugins(beat::plugin);
}

#[derive(Asset, Reflect, Serialize, Deserialize)]
struct MusicConfig {
    bpm: f32,
}

impl Config for MusicConfig {
    const PATH: &'static str = "config/music.ron";
    const EXTENSION: &'static str = "music.ron";

    fn on_load(&mut self, world: &mut World) {
        world
            .resource_mut::<BeatTimer>()
            .0
            .set_duration(Duration::from_secs_f32(60.0 / self.bpm));
    }
}
