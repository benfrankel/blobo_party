use std::f32::consts::TAU;

use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use rand::Rng as _;
use serde::Deserialize;
use serde::Serialize;

use crate::core::PostColorSet;
use crate::core::PostTransformSet;
use crate::core::UpdateSet;
use crate::game::GameRoot;
use crate::screen::playing::PlayingAssets;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(ConfigHandle<SpotlightConfig>, Spotlight)>();
}

#[derive(Asset, Reflect, Serialize, Deserialize)]
pub struct SpotlightConfig {
    pub rotation_rate_lo: f32,
    pub rotation_rate_hi: f32,
    pub color_loop_rate_lo: f32,
    pub color_loop_rate_hi: f32,
    pub color_loop: Vec<Color>,
}

impl Config for SpotlightConfig {
    const PATH: &'static str = "config/spotlight.ron";
    const EXTENSION: &'static str = "spotlight.ron";
}

impl SpotlightConfig {
    fn color(&self, t: f32) -> Color {
        let n = self.color_loop.len();
        let t = t * n as f32;
        let lo = t as usize;
        let hi = if lo + 1 < n { lo + 1 } else { 0 };
        let t = t.fract();

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

pub fn spotlight_lamp(mut entity: EntityWorldMut) {
    let parent = entity.world().resource::<GameRoot>().vfx;
    let texture = entity
        .world()
        .resource::<PlayingAssets>()
        .spotlight_lamp
        .clone();

    entity
        .insert((
            Name::new("SpotlightLamp"),
            SpriteBundle {
                texture,
                transform: Transform::from_xyz(0.0, 0.0, -5.0),
                ..default()
            },
        ))
        .set_parent(parent)
        .with_children(|children| {
            let spawn_count = rand::thread_rng().gen_range(4..=16);
            for _ in 0..spawn_count {
                children.spawn_with(spotlight);
            }
        });
}

fn spotlight(entity: Entity, world: &mut World) {
    let (playing_assets, config) =
        SystemState::<(Res<PlayingAssets>, ConfigRef<SpotlightConfig>)>::new(world).get(world);
    let config = r!(config.get());
    let texture = playing_assets.spotlight.clone();

    let mut rng = rand::thread_rng();

    let initial_rotation = Quat::from_rotation_z(rng.gen_range(0.0..=TAU));
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
