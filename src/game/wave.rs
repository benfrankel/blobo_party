use bevy::ecs::system::EntityCommand;
use bevy::prelude::*;
use pyri_state::prelude::*;
use rand::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::core::camera::CameraRoot;
use crate::core::UpdateSet;
use crate::game::actor::enemy::enemy;
use crate::game::actor::level::Level;
use crate::game::actor::ActorConfig;
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
}

impl Config for WaveConfig {
    const PATH: &'static str = "config/wave.ron";
    const EXTENSION: &'static str = "wave.ron";
}

#[allow(unused)]
#[derive(Asset, Reflect, Serialize, Deserialize, Default)]
struct SpawnModifiers {
    // TODO: Fill out with modifiers to apply to actor on spawn.
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
    actor_config: ConfigRef<ActorConfig>,
    camera_root: Res<CameraRoot>,
    camera_query: Query<&GlobalTransform>,
    mut wave_query: Query<(&mut Wave, &Selection)>,
    level_query: Query<&Level>,
) {
    let config = r!(config.get());
    let actor_config = r!(actor_config.get());
    let camera_gt = r!(camera_query.get(camera_root.primary));
    let center = camera_gt.translation().xy();

    let mut rng = rand::thread_rng();
    for (mut wave, selection) in &mut wave_query {
        let level = c!(level_query.get(selection.0));
        let level = level.current;

        wave.0 = wave.0.wrapping_add(1);
        if wave.0 % config.spawn_cadence != 0 {
            return;
        }

        let enemy_pool = actor_config
            .enemies
            .iter()
            .filter(|(_, enemy)| enemy.min_level <= level && level <= enemy.max_level)
            .collect::<Vec<_>>();

        let spawn_count = (level / config.spawn_count_scale).clamp(1, config.max_spawn_count);
        for _ in 0..spawn_count {
            let enemy_key = c!(enemy_pool.choose_weighted(&mut rng, |(_, enemy)| enemy.weight)).0;
            let offset =
                Annulus::new(config.min_distance, config.max_distance).sample_interior(&mut rng);
            let spawn_point = center + offset;

            commands
                .spawn_with(enemy(enemy_key))
                .insert(Transform::from_translation(spawn_point.extend(0.0)));
        }
    }
}

pub fn wave(player: Entity) -> impl EntityCommand<World> {
    move |mut entity: EntityWorldMut| {
        entity.insert((Name::new("Wave"), Wave::default(), Selection(player)));
    }
}
