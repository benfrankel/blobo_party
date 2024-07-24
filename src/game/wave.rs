use std::f32::consts::TAU;

use bevy::prelude::*;
use bevy::utils::HashMap;
use pyri_state::prelude::*;
use rand::seq::IteratorRandom;
use rand::Rng;
use serde::Deserialize;
use serde::Serialize;

use crate::core::camera::CameraRoot;
use crate::core::UpdateSet;
use crate::game::actor::enemy::enemy;
use crate::game::level::Level;
use crate::game::music::beat::on_beat;
use crate::screen::Screen;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(ConfigHandle<WaveConfig>, Wave)>();
}

#[derive(Asset, Reflect, Serialize, Deserialize)]
pub struct WaveConfig {
    pub spawn_cadence: usize,
    pub min_distance: f32,
    pub max_distance: f32,
    pub spawn_count_scale: usize,
    pub max_spawn_count: usize,
    pub enemies: HashMap<String, Vec<SpawnInfo>>,
}

impl Config for WaveConfig {
    const PATH: &'static str = "config/wave.ron";
    const EXTENSION: &'static str = "wave.ron";

    fn on_load(&mut self, _world: &mut World) {}
}

#[derive(Asset, Reflect, Serialize, Deserialize)]
pub struct SpawnInfo {
    #[serde(default)]
    condition: Vec<SpawnCondition>,
    #[serde(default)]
    modifiers: SpawnModifiers,
}

#[derive(Default, Asset, Reflect, Serialize, Deserialize)]
struct SpawnModifiers {
    // TODO: Fill out with modifiers to apply to actor on spawn.
}

#[derive(Reflect, Serialize, Deserialize)]
enum SpawnCondition {
    GreaterThan(usize),
    LessThan(usize),
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
struct Wave(usize);

impl Configure for Wave {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_resource::<Self>();
        app.add_systems(
            Update,
            Screen::Playing.on_update(
                spawn_wave_enemies
                    .in_set(UpdateSet::Update)
                    .run_if(on_beat(2)),
            ),
        );
    }
}

fn spawn_wave_enemies(
    mut commands: Commands,
    mut wave: ResMut<Wave>,
    camera_root: Res<CameraRoot>,
    camera_query: Query<&GlobalTransform>,
    level: Res<Level>,
    config_handle: Res<ConfigHandle<WaveConfig>>,
    config: Res<Assets<WaveConfig>>,
) {
    let config = r!(config.get(&config_handle.0));
    let camera_gt = r!(camera_query.get(camera_root.primary));
    let center = camera_gt.translation().xy();

    wave.0 = wave.0.wrapping_add(1);
    if wave.0 % config.spawn_cadence != 0 {
        return;
    }

    let mut rng = rand::thread_rng();
    let spawn_count = (level.current / config.spawn_count_scale)
        .max(1)
        .min(config.max_spawn_count);
    for _ in 0..spawn_count {
        let available_actors = config
            .enemies
            .iter()
            // Flatten each spawn_info with its actor key.
            .flat_map(|(key, info)| info.iter().map(|info| (key.clone(), info)))
            .filter(|(_, info)| {
                info.condition.is_empty() || any_condition_met(&info.condition, &level.current)
            });

        if let Some((key, _)) = available_actors.choose(&mut rng) {
            let direction = Vec2::from_angle(rng.gen_range(0.0..=TAU));
            let distance = rng.gen_range(config.min_distance..config.max_distance);
            let spawn_point = center + direction * distance;

            commands
                .spawn_with(enemy(key))
                .insert(Transform::from_translation(spawn_point.extend(0.0)));
        }
    }
}

fn any_condition_met(conditions: &[SpawnCondition], current_level: &usize) -> bool {
    conditions.iter().all(|condition| match condition {
        SpawnCondition::GreaterThan(value) => current_level > value,
        SpawnCondition::LessThan(value) => current_level < value,
    })
}
