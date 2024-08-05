use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use pyri_state::prelude::*;

use crate::core::pause::Pause;
use crate::core::UpdateSet;
use crate::game::audio::AudioConfig;
use crate::screen::playing::PlayingAssets;
use crate::screen::Screen;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(MusicHandle, Beat)>();
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct MusicHandle(pub Handle<AudioInstance>);

impl Configure for MusicHandle {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_resource::<Self>();
        app.add_systems(
            StateFlush,
            Pause.on_edge(
                // TODO: Unfortunate that this run condition is necessary...
                unpause_music.run_if(Screen::Playing.will_enter()),
                pause_music,
            ),
        );
    }
}

pub fn stop_music(
    music_handle: Res<MusicHandle>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    let music = r!(audio_instances.get_mut(&music_handle.0));
    music.stop(AudioTween::default());
}

pub fn start_music(
    config: ConfigRef<AudioConfig>,
    audio: Res<Audio>,
    assets: Res<PlayingAssets>,
    mut music_handle: ResMut<MusicHandle>,
) {
    let config = r!(config.get());
    music_handle.0 = audio
        .play(assets.music.clone())
        .with_volume(config.music_volume)
        .loop_from(config.music_loop_start)
        .loop_until(config.music_loop_end)
        .handle();
}

pub fn pause_music(
    music_handle: Res<MusicHandle>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    let music = r!(audio_instances.get_mut(&music_handle.0));
    music.pause(AudioTween::default());
}

fn unpause_music(
    music_handle: Res<MusicHandle>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    let music = r!(audio_instances.get_mut(&music_handle.0));
    music.resume(AudioTween::default());
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct Beat {
    /// The position of the music audio instance last frame.
    last_position: f64,
    /// The total number of eighth-beats counted.
    pub total: usize,
    /// The number of new eighth-beats finished this tick (usually 0 or 1).
    pub this_tick: usize,
}

impl Configure for Beat {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_resource::<Self>();
        app.add_systems(Update, update_beat.in_set(UpdateSet::SyncEarly));
    }
}

/// A helper function to count how many beats occur in the music between two positions.
fn count_beats(config: &AudioConfig, lo: f64, hi: f64) -> usize {
    let lo_beats = (lo - config.music_zeroth_beat).max(0.0) * config.music_bpm * 8.0 / 60.0;
    let hi_beats = (hi - config.music_zeroth_beat).max(0.0) * config.music_bpm * 8.0 / 60.0;
    hi_beats as usize - lo_beats as usize
}

fn update_beat(
    config: ConfigRef<AudioConfig>,
    music_handle: Res<MusicHandle>,
    audio_instances: ResMut<Assets<AudioInstance>>,
    mut beat: ResMut<Beat>,
) {
    let config = r!(config.get());
    let music = r!(audio_instances.get(&music_handle.0));
    let position = rq!(music.state().position());

    beat.this_tick = if beat.last_position <= position {
        count_beats(config, beat.last_position, position)
    } else {
        count_beats(config, beat.last_position, config.music_loop_end)
            + count_beats(config, config.music_loop_start, position)
    };
    beat.total += beat.this_tick;
    beat.last_position = position;
}

/// A run condition to run a system every `n` eighth-beats.
pub fn on_beat(n: usize) -> impl Fn(Res<Beat>) -> bool {
    move |beat| {
        let hi = beat.total;
        let lo = hi - beat.this_tick;
        hi / n > lo / n
    }
}

/// A run condition to run a system every `n` beats.
pub fn on_full_beat(n: usize) -> impl Fn(Res<Beat>) -> bool {
    on_beat(8 * n)
}
