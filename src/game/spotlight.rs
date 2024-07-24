use bevy::math::vec3;
use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::screen::playing::PlayingAssets;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<IsSpotlight>();
}

/// A marker component for spotlight entities.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct IsSpotlight;

impl Configure for IsSpotlight {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        // TODO: Animate spotlight: rotation, hue, alpha.
        // TODO: Temporary. Create an actual spotlight spawner.
        app.add_systems(Update, spawn_spotlight.run_if(run_once()));
    }
}

fn spawn_spotlight(mut commands: Commands) {
    commands
        .spawn_with(spotlight)
        .insert(Transform::from_translation(vec3(0.0, 0.0, -5.0)));
}

fn spotlight(mut entity: EntityWorldMut) {
    let texture = entity.world().resource::<PlayingAssets>().spotlight.clone();

    entity.insert((
        Name::new("Spotlight"),
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                anchor: Anchor::CenterLeft,
                ..default()
            },
            texture,
            ..default()
        },
        IsSpotlight,
    ));
}
