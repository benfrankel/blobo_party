use bevy::ecs::system::EntityCommand;
use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::core::UpdateSet;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(Health, ConfigHandle<HealthBarConfig>, HealthBar)>();
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Health {
    pub max: f32,
    pub current: f32,
}

impl Configure for Health {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { max, current: max }
    }
}

#[derive(Asset, Reflect, Serialize, Deserialize)]
pub struct HealthBarConfig {
    pub color_ramp: Vec<Color>,
}

impl Config for HealthBarConfig {
    const PATH: &'static str = "config/health_bar.ron";
    const EXTENSION: &'static str = "health_bar.ron";
}

impl HealthBarConfig {
    fn color(&self, t: f32) -> Color {
        let t = t * (self.color_ramp.len() - 1) as f32;
        let lo = t as usize;
        let hi = lo + 1;
        if hi >= self.color_ramp.len() {
            self.color_ramp[self.color_ramp.len() - 1]
        } else {
            self.color_ramp[lo].mix(&self.color_ramp[hi], t.fract())
        }
    }
}

/// Reads from the `Health` component on its parent entity.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct HealthBar {
    pub size: Vec2,
}

impl Configure for HealthBar {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(Update, update_health_bar.in_set(UpdateSet::SyncLate));
    }
}

fn update_health_bar(
    config_handle: Res<ConfigHandle<HealthBarConfig>>,
    config: Res<Assets<HealthBarConfig>>,
    health_query: Query<&Health>,
    mut health_bar_query: Query<(&HealthBar, &Parent, &mut Sprite)>,
) {
    let config = r!(config.get(&config_handle.0));

    for (health_bar, parent, mut sprite) in &mut health_bar_query {
        let health = c!(health_query.get(parent.get()));
        let t = health.current / health.max;

        sprite.custom_size = Some(Vec2::new(t * health_bar.size.x, health_bar.size.y));
        sprite.color = config.color(t);
    }
}

impl EntityCommand for HealthBar {
    fn apply(self, id: Entity, world: &mut World) {
        world
            .entity_mut(id)
            .insert((Name::new("HealthBar"), SpriteBundle::default(), self));
    }
}
