use avian2d::prelude::*;
use bevy::ecs::system::EntityCommand;
use bevy::ecs::system::SystemState;
use bevy::prelude::*;

use crate::game::actor::facing::FacePlayer;
use crate::game::actor::faction::Faction;
use crate::game::actor::ActorConfig;
use crate::game::combat::death::DeathSfx;
use crate::game::combat::death::DespawnOnDeath;
use crate::game::GameLayer;
use crate::game::GameRoot;
use crate::screen::playing::PlayingAssets;
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

pub fn enemy(key: impl Into<String>) -> impl EntityCommand {
    let key = key.into();
    move |entity: Entity, world: &mut World| {
        let (actor, parent, sfx_death) = {
            let (config, game_root, assets) =
                SystemState::<(ConfigRef<ActorConfig>, Res<GameRoot>, Res<PlayingAssets>)>::new(
                    world,
                )
                .get(world);
            let config = r!(config.get());
            let actor = r!(config.enemies.get(&key)).clone();

            (actor, game_root.players, assets.sfx_kick.clone())
        };

        world
            .entity_mut(entity)
            .add(actor)
            .insert((
                IsEnemy,
                Faction::Enemy,
                CollisionLayers::new(GameLayer::Enemy, LayerMask::ALL),
                FacePlayer,
                DeathSfx(sfx_death, 0.5),
                // TODO: Despawn when death animation is finished, instead.
                DespawnOnDeath,
            ))
            .set_parent(parent);
    }
}
