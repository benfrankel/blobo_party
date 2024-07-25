use std::f32::consts::TAU;

use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use interpolation::Ease as _;
use rand::Rng as _;
use serde::Deserialize;
use serde::Serialize;

use super::cleanup::DespawnRadiusSq;
use crate::core::camera::CameraRoot;
use crate::core::PostColorSet;
use crate::core::PostTransformSet;
use crate::core::UpdateSet;
use crate::game::music::beat::on_beat;
use crate::game::GameRoot;
use crate::screen::playing::PlayingAssets;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(
        ConfigHandle<SpotlightConfig>,
        Spotlight,
        IsSpotlightLamp,
        IsSpotlightLampSpawner,
    )>();
}

#[derive(Asset, Reflect, Serialize, Deserialize)]
pub struct SpotlightConfig {
    // Lamp spawner:
    pub spawn_radius_lo: f32,
    pub spawn_radius_hi: f32,
    pub despawn_radius: f32,
    pub spawn_cap: usize,

    // Lamp:
    pub light_count_lo: usize,
    pub light_count_hi: usize,

    // Spotlight:
    pub rotation_rate_lo: f32,
    pub rotation_rate_hi: f32,
    pub color_loop_rate_lo: f32,
    pub color_loop_rate_hi: f32,
    pub color_loop: Vec<Color>,
    pub alpha_multiplier: f32,
}

impl Config for SpotlightConfig {
    const PATH: &'static str = "config/spotlight.ron";
    const EXTENSION: &'static str = "spotlight.ron";

    fn on_load(&mut self, _world: &mut World) {
        for color in &mut self.color_loop {
            color.set_alpha(color.alpha() * self.alpha_multiplier);
        }
    }
}

impl SpotlightConfig {
    // TODO: This panicked once with "len is 7 but index is 7". I have NO CLUE how that's possible.
    fn color(&self, t: f32) -> Color {
        let n = self.color_loop.len();
        let t = t * n as f32;
        let lo = t as usize;
        let hi = if lo + 1 < n { lo + 1 } else { 0 };
        let t = t.fract().quadratic_in_out();

        self.color_loop[lo].better_mix(&self.color_loop[hi], t)
    }
}

/// A marker component for spotlight entities.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Spotlight {
    pub rotation_rate: f32,
    pub color_loop_rate: f32,
    pub color_loop_t: f32,
}

impl Configure for Spotlight {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(Update, tick_spotlight.in_set(UpdateSet::TickTimers));
        app.add_systems(
            PostUpdate,
            (
                update_spotlight_rotation.in_set(PostTransformSet::Blend),
                update_spotlight_color.in_set(PostColorSet::Blend),
            ),
        );
    }
}

fn tick_spotlight(time: Res<Time>, mut spotlight_query: Query<&mut Spotlight>) {
    let dt = time.delta_seconds();
    for mut spotlight in &mut spotlight_query {
        spotlight.color_loop_t += spotlight.color_loop_rate * dt;
        spotlight.color_loop_t = spotlight.color_loop_t.rem_euclid(1.0);
    }
}

fn update_spotlight_rotation(
    time: Res<Time>,
    mut spotlight_query: Query<(&Spotlight, &mut Transform)>,
) {
    let dt = time.delta_seconds();
    for (spotlight, mut transform) in &mut spotlight_query {
        transform.rotate_z(spotlight.rotation_rate * dt);
    }
}

fn update_spotlight_color(
    config: ConfigRef<SpotlightConfig>,
    mut spotlight_query: Query<(&Spotlight, &mut Sprite)>,
) {
    let config = r!(config.get());
    for (spotlight, mut sprite) in &mut spotlight_query {
        sprite.color = config.color(spotlight.color_loop_t);
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct IsSpotlightLampSpawner;

impl Configure for IsSpotlightLampSpawner {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            spawn_spotlight_lamps
                .in_set(UpdateSet::Update)
                .run_if(not(any_with_component::<Spotlight>).or_else(on_beat(1))),
        );
    }
}

fn spawn_spotlight_lamps(
    mut commands: Commands,
    config: ConfigRef<SpotlightConfig>,
    camera_root: Res<CameraRoot>,
    camera_query: Query<&GlobalTransform>,
    spawner_query: Query<(), With<IsSpotlightLampSpawner>>,
    lamp_query: Query<(), With<IsSpotlightLamp>>,
) {
    let config = r!(config.get());
    let camera_gt = r!(camera_query.get(camera_root.primary));
    let center = camera_gt.translation().xy();

    let mut rng = rand::thread_rng();
    for () in &spawner_query {
        let lamp_count = lamp_query.iter().len();
        let spawn_count = config.spawn_cap.saturating_sub(lamp_count);
        if spawn_count == 0 {
            return;
        }
        let spawn_radius_lo = if lamp_count > 0 {
            config.spawn_radius_lo
        } else {
            0.0
        };

        for _ in 0..spawn_count {
            let offset =
                Annulus::new(spawn_radius_lo, config.spawn_radius_hi).sample_interior(&mut rng);
            let spawn_point = center + offset;

            commands
                .spawn_with(spotlight_lamp)
                .insert(Transform::from_translation(spawn_point.extend(-5.0)));
        }
    }
}

pub fn spotlight_lamp_spawner(mut entity: EntityWorldMut) {
    entity.insert((Name::new("SpotlightLampSpawner"), IsSpotlightLampSpawner));
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct IsSpotlightLamp;

impl Configure for IsSpotlightLamp {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

fn spotlight_lamp(entity: Entity, world: &mut World) {
    let (config, game_root, playing_assets) = SystemState::<(
        ConfigRef<SpotlightConfig>,
        Res<GameRoot>,
        Res<PlayingAssets>,
    )>::new(world)
    .get(world);
    let config = r!(config.get());
    let despawn_radius = config.despawn_radius;
    let light_count = rand::thread_rng().gen_range(config.light_count_lo..=config.light_count_hi);
    let parent = game_root.vfx;
    let texture = playing_assets.spotlight_lamp.clone();

    world
        .entity_mut(entity)
        .insert((
            Name::new("SpotlightLamp"),
            SpriteBundle {
                texture,
                ..default()
            },
            DespawnRadiusSq::new(despawn_radius),
            IsSpotlightLamp,
        ))
        .set_parent(parent)
        .with_children(|children| {
            for _ in 0..light_count {
                children.spawn_with(spotlight);
            }
        });
}

fn spotlight(entity: Entity, world: &mut World) {
    let (config, playing_assets) =
        SystemState::<(ConfigRef<SpotlightConfig>, Res<PlayingAssets>)>::new(world).get(world);
    let config = r!(config.get());
    let texture = playing_assets.spotlight.clone();

    let mut rng = rand::thread_rng();

    let initial_rotation = Quat::from_rotation_z(rng.gen_range(0.0..TAU));
    let rotation_rate = if rng.gen::<bool>() { -1.0 } else { 1.0 }
        * rng.gen_range(config.rotation_rate_lo..=config.rotation_rate_hi);

    let color_loop_t = rng.gen_range(0.0..1.0);
    let color_loop_rate = if rng.gen::<bool>() { -1.0 } else { 1.0 }
        * rng.gen_range(config.color_loop_rate_lo..=config.color_loop_rate_hi);

    world.entity_mut(entity).insert((
        Name::new("Spotlight"),
        SpriteBundle {
            sprite: Sprite {
                color: Oklcha::default().into(),
                anchor: Anchor::CenterLeft,
                ..default()
            },
            texture,
            transform: Transform::from_xyz(0.0, 0.0, 10.0).with_rotation(initial_rotation),
            ..default()
        },
        Spotlight {
            rotation_rate,
            color_loop_rate,
            color_loop_t,
        },
    ));
}
