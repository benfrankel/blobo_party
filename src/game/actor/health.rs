use bevy::ecs::system::EntityCommand;
use bevy::prelude::*;

use crate::core::UpdateSet;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(Health, HealthBar)>();
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
    health_query: Query<&Health>,
    mut health_bar_query: Query<(&HealthBar, &Parent, &mut Sprite)>,
) {
    for (health_bar, parent, mut sprite) in &mut health_bar_query {
        let health = c!(health_query.get(parent.get()));
        let t = health.current / health.max;

        sprite.custom_size = Some(Vec2::new(t * health_bar.size.x, health_bar.size.y));
        // TODO: Color ramp.
        sprite.color = Color::srgb(0.0, 1.0, 0.0);
    }
}

impl EntityCommand for HealthBar {
    fn apply(self, id: Entity, world: &mut World) {
        world
            .entity_mut(id)
            .insert((Name::new("HealthBar"), SpriteBundle::default(), self));
    }
}
