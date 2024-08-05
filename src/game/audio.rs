pub mod music;

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::game::audio::music::MusicHandle;
use crate::screen::playing::PlayingAssets;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<ConfigHandle<AudioConfig>>();

    app.add_plugins(music::plugin);
}

#[derive(Asset, Reflect, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AudioConfig {
    pub global_volume: f64,

    pub music_volume: f64,
    /// The precise beats-per-minute of the music.
    pub music_bpm: f64,
    /// The position (in seconds) of the zeroth beat.
    pub music_zeroth_beat: f64,
    pub music_loop_start: f64,
    pub music_loop_end: f64,
}

impl Config for AudioConfig {
    const PATH: &'static str = "config/audio.ron";
    const EXTENSION: &'static str = "audio.ron";

    fn on_load(&mut self, world: &mut World) {
        world.resource::<Audio>().set_volume(self.global_volume);

        if !world
            .resource::<Assets<AudioInstance>>()
            .contains(&world.resource::<MusicHandle>().0)
        {
            let music_handle = world
                .resource::<Audio>()
                .play(world.resource::<PlayingAssets>().music.clone())
                .with_volume(self.music_volume)
                .loop_from(self.music_loop_start)
                .loop_until(self.music_loop_end)
                .paused()
                .handle();
            world.resource_mut::<MusicHandle>().0 = music_handle;
        }
    }
}
