use avian2d::prelude::*;
use bevy::ecs::system::EntityCommand;
use bevy::ecs::system::SystemState;
use bevy::math::vec2;
use bevy::math::vec3;
use bevy::prelude::*;

use crate::core::camera::CameraRoot;
use crate::core::camera::SmoothFollow;
use crate::game::actor::attack::input::attack_action;
use crate::game::actor::facing::FaceCursor;
use crate::game::actor::facing::FacingIndicator;
use crate::game::actor::faction::Faction;
use crate::game::actor::movement::input::movement_action;
use crate::game::actor::ActorConfig;
use crate::game::combat::death::DeathSfx;
use crate::game::combat::hit::Hitbox;
use crate::game::combat::hit::HurtSfx;
use crate::game::combat::knockback::HitboxKnockback;
use crate::game::GameLayer;
use crate::game::GameRoot;
use crate::screen::playing::PlayingAssets;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<IsPlayer>();
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct IsPlayer;

impl Configure for IsPlayer {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

pub fn player(key: impl Into<String>) -> impl EntityCommand {
    let key = key.into();

    move |entity: Entity, world: &mut World| {
        let (actor, parent, camera, sfx_hurt, sfx_death) = {
            let (config, game_root, camera_root, assets) = SystemState::<(
                ConfigRef<ActorConfig>,
                Res<GameRoot>,
                Res<CameraRoot>,
                Res<PlayingAssets>,
            )>::new(world)
            .get(world);
            let config = r!(config.get());
            let actor = r!(config.players.get(&key)).clone();

            (
                actor,
                game_root.players,
                camera_root.primary,
                assets.sfx_player_hurt.clone(),
                assets.sfx_restart.clone(),
            )
        };

        world
            .entity_mut(entity)
            .add(actor)
            .insert((
                IsPlayer,
                Faction::Player,
                CollisionLayers::new(GameLayer::Player, LayerMask::ALL),
                ColliderDensity(5.0),
                FaceCursor,
                // Contact hitbox was for testing, but it's funny, so I'm leaving it in.
                Hitbox,
                HitboxKnockback(150.0, false),
                HurtSfx(sfx_hurt, 1.8),
                DeathSfx(sfx_death, 1.0),
            ))
            .set_parent(parent)
            .with_children(|children| {
                children
                    .spawn_with(FacingIndicator {
                        offset: vec2(6.0, 5.0),
                    })
                    .insert(Transform::from_translation(vec3(0.0, -0.5, 2.0)));
            });

        // Allow manual movement / attack input in dev builds.
        #[cfg(feature = "dev")]
        world
            .entity_mut(entity)
            .add(movement_action)
            .add(attack_action);

        r!(world.entity_mut(camera).get_mut::<SmoothFollow>()).target = entity;
    }
}
