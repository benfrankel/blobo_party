use bevy::math::vec3;
use bevy::prelude::*;
use bevy::utils::HashMap;
use pyri_state::prelude::*;
use rand::seq::IteratorRandom;
use rand::Rng;
use serde::Deserialize;
use serde::Serialize;

use crate::core::UpdateSet;
use crate::game::actor::enemy::enemy;
use crate::game::actor::player::IsPlayer;
use crate::game::level::Level;
use crate::game::music::beat::on_beat;
use crate::screen::Screen;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<ConfigHandle<SpawnerConfig>>();
    app.init_resource::<Spawner>();
    app.add_systems(
        Update,
        Screen::Playing.on_update((spawn_actors.in_set(UpdateSet::Update).run_if(on_beat(2)),)),
    );
}

#[derive(Default, Resource)]
struct Spawner(usize);

#[derive(Asset, Reflect, Serialize, Deserialize)]
pub struct SpawnerConfig {
    pub spawn_cadence: usize,
    pub min_distance: f32,
    pub max_distance: f32,
    pub spawn_count_scale: usize,
    pub max_spawn_count: usize,
    pub actors: HashMap<String, Vec<SpawnInfo>>,
}

impl Config for SpawnerConfig {
    const PATH: &'static str = "config/spawner.ron";
    const EXTENSION: &'static str = "spawner.ron";

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
    // TODO: fill out with modifiers to apply to actor on spawn
}

#[derive(Reflect, Serialize, Deserialize)]
enum SpawnCondition {
    GreaterThan(usize),
    LessThan(usize),
}

fn spawn_actors(
    mut commands: Commands,
    mut spawner: ResMut<Spawner>,
    player: Query<&Transform, With<IsPlayer>>,
    level: Res<Level>,
    config_handle: Res<ConfigHandle<SpawnerConfig>>,
    config: Res<Assets<SpawnerConfig>>,
) {
    // if there are multiple players, just grab one
    let Some(player_transform) = player.iter().last() else {
        return;
    };
    let config = r!(config.get(&config_handle.0));

    spawner.0 = spawner.0.wrapping_add(1);
    if spawner.0 % config.spawn_cadence != 0 {
        return;
    }

    let count = (level.current / config.spawn_count_scale)
        .max(1)
        .min(config.max_spawn_count);
    for _ in 0..count {
        let available_actors = config
            .actors
            .iter()
            // flatten each spawn_info with its actor key
            .flat_map(|(key, info)| info.iter().map(|info| (key.clone(), info)))
            .filter(|(_, info)| {
                info.condition.is_empty() || any_condition_met(&info.condition, &level.current)
            });

        let mut rng = rand::thread_rng();
        if let Some((key, _)) = available_actors.choose(&mut rng) {
            let direction = vec3(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
            let distance = rng.gen_range(config.min_distance..config.max_distance);
            let spawn_point = (direction * distance) + player_transform.translation;

            commands
                .spawn_with(enemy(key))
                .insert(TransformBundle::from_transform(
                    Transform::from_translation(spawn_point),
                ));
        }
    }
}

fn any_condition_met(conditions: &[SpawnCondition], current_level: &usize) -> bool {
    conditions.iter().all(|condition| match condition {
        SpawnCondition::GreaterThan(value) => current_level > value,
        SpawnCondition::LessThan(value) => current_level < value,
    })
}
