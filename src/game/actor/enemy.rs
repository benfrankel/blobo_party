use avian2d::prelude::*;
use bevy::ecs::system::EntityCommand;
use bevy::prelude::*;

use crate::game::actor::actor;
use crate::game::actor::facing::FacePlayer;
use crate::game::actor::faction::Faction;
use crate::game::actor::ActorConfig;
use crate::game::GameLayer;
use crate::game::GameRoot;
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
        let parent = entity.world().resource::<GameRoot>().enemies;
        let config_handle = entity.world().resource::<ConfigHandle<ActorConfig>>();
        let config = r!(entity
            .world()
            .resource::<Assets<ActorConfig>>()
            .get(&config_handle.0),);

        entity
            .add(actor(r!(config.enemies.get(&key))))
            .insert((
                IsEnemy,
                Faction::Enemy,
                CollisionLayers::new(GameLayer::Enemy, LayerMask::ALL),
                FacePlayer,
            ))
            .set_parent(parent);
    }
}
