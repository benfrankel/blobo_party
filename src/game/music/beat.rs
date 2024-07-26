use std::time::Duration;

use bevy::prelude::*;
use pyri_state::prelude::*;

use crate::core::pause::Pause;
use crate::core::UpdateSet;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(BeatTimer, Beat)>();
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct BeatTimer(pub Timer);

impl Configure for BeatTimer {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_resource::<Self>();
        app.add_systems(Update, tick_beat_timer.in_set(UpdateSet::TickTimers));
        app.add_systems(
            StateFlush,
            Pause.on_edge(unpause_beat_timer, pause_beat_timer),
        );
    }
}

impl Default for BeatTimer {
    fn default() -> Self {
        Self(Timer::new(
            Duration::from_secs_f32(60.0 / 122.0),
            TimerMode::Repeating,
        ))
    }
}

fn tick_beat_timer(time: Res<Time>, mut beat_timer: ResMut<BeatTimer>) {
    beat_timer.0.tick(time.delta());
}

fn unpause_beat_timer(mut beat_timer: ResMut<BeatTimer>) {
    beat_timer.0.unpause();
}

fn pause_beat_timer(mut beat_timer: ResMut<BeatTimer>) {
    beat_timer.0.pause();
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct Beat {
    pub total: usize,
    pub this_tick: usize,
}

impl Configure for Beat {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_resource::<Self>();
        app.add_systems(Update, update_beat.in_set(UpdateSet::SyncEarly));
    }
}

fn update_beat(beat_timer: Res<BeatTimer>, mut beat: ResMut<Beat>) {
    beat.this_tick = beat_timer.0.times_finished_this_tick() as usize;
    beat.total += beat.this_tick;
}

/// A run condition to run a system every `n` beats.
pub fn on_beat(n: usize) -> impl Fn(Res<Beat>) -> bool {
    move |beat| {
        let hi = beat.total;
        let lo = hi - beat.this_tick;
        hi / n > lo / n
    }
}
