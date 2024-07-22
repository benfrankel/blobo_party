use avian2d::prelude::*;
use bevy::ecs::system::EntityCommand;
use bevy::prelude::*;

use crate::game::actor::actor;
use crate::game::actor::facing::FacePlayer;
use crate::game::GameLayer;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<IsEnemy>();
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct IsEnemy;

impl Configure for IsEnemy {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

pub fn enemy(key: impl Into<String>) -> impl EntityCommand<World> {
    let key = key.into();
    move |mut entity: EntityWorldMut| {
        entity.add(actor(key)).insert((
            IsEnemy,
            CollisionLayers::new(GameLayer::Enemy, LayerMask::ALL),
            FacePlayer,
        ));
    }
}
