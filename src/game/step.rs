use std::time::Duration;

use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::core::UpdateSet;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(ConfigHandle<StepConfig>, StepTimer, Step)>();
}

#[derive(Asset, Reflect, Serialize, Deserialize)]
struct StepConfig {
    duration_millis: u64,
}

impl Config for StepConfig {
    const PATH: &'static str = "config/step.ron";

    const EXTENSION: &'static str = "step.ron";

    fn on_load(&mut self, world: &mut World) {
        world
            .resource_mut::<StepTimer>()
            .0
            .set_duration(Duration::from_millis(self.duration_millis));
    }
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct StepTimer(pub Timer);

impl Configure for StepTimer {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_resource::<Self>();
        app.add_systems(Update, tick_step_timer.in_set(UpdateSet::TickTimers));
    }
}

impl Default for StepTimer {
    fn default() -> Self {
        Self(Timer::new(Duration::from_millis(125), TimerMode::Repeating))
    }
}

fn tick_step_timer(time: Res<Time>, mut step_timer: ResMut<StepTimer>) {
    step_timer.0.tick(time.delta());
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct Step(pub usize);

impl Configure for Step {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_resource::<Self>();
        app.add_systems(Update, update_step.in_set(UpdateSet::SyncEarly));
        app.add_systems(
            Update,
            (|step: Res<Step>| println!("Step: {}", step.0))
                .in_set(UpdateSet::Update)
                .run_if(on_step(2)),
        );
    }
}

fn update_step(step_timer: Res<StepTimer>, mut step: ResMut<Step>) {
    step.0 += step_timer.0.times_finished_this_tick() as usize;
}

/// A run condition to run a system every `n` steps.
pub fn on_step(n: usize) -> impl Fn(Res<StepTimer>, Res<Step>) -> bool {
    move |step_timer, step| {
        let hi = step.0;
        let lo = hi - step_timer.0.times_finished_this_tick() as usize;
        hi / n > lo / n
    }
}
