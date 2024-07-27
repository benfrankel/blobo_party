use bevy::ecs::system::EntityCommand;
use bevy::prelude::*;
use bevy::utils::HashMap;
use pyri_state::prelude::*;
use rand::seq::IteratorRandom;
use serde::Deserialize;
use serde::Serialize;

use crate::core::camera::CameraRoot;
use crate::core::UpdateSet;
use crate::game::actor::enemy::enemy;
use crate::game::actor::level::Level;
use crate::game::audio::music::on_full_beat;
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

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
struct Wave(usize);

impl Configure for Wave {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            Screen::Playing.on_update(
                spawn_wave_enemies
                    .in_set(UpdateSet::Update)
                    .run_if(on_full_beat(1)),
            ),
        );
    }
}

fn spawn_wave_enemies(
    mut commands: Commands,
    config: ConfigRef<WaveConfig>,
    camera_root: Res<CameraRoot>,
    camera_query: Query<&GlobalTransform>,
    mut wave_query: Query<(&mut Wave, &Selection)>,
    level_query: Query<&Level>,
) {
    let config = r!(config.get());
    let camera_gt = r!(camera_query.get(camera_root.primary));
    let center = camera_gt.translation().xy();

    for (mut wave, selection) in &mut wave_query {
        let level = c!(level_query.get(selection.0));

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
                let offset = Annulus::new(config.min_distance, config.max_distance)
                    .sample_interior(&mut rng);
                let spawn_point = center + offset;

                commands
                    .spawn_with(enemy(key))
                    .insert(Transform::from_translation(spawn_point.extend(0.0)));
            }
        }
    }
}

fn any_condition_met(conditions: &[SpawnCondition], current_level: &usize) -> bool {
    conditions.iter().all(|condition| match condition {
        SpawnCondition::GreaterThan(value) => current_level > value,
        SpawnCondition::LessThan(value) => current_level < value,
    })
}

pub fn wave(player: Entity) -> impl EntityCommand<World> {
    move |mut entity: EntityWorldMut| {
        entity.insert((Name::new("Wave"), Wave::default(), Selection(player)));
    }
}
