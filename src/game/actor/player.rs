use avian2d::prelude::*;
use bevy::math::vec2;
use bevy::math::vec3;
use bevy::prelude::*;

use crate::game::actor::actor_helper;
use crate::game::actor::attack::input::attack_action;
use crate::game::actor::facing::FaceCursor;
use crate::game::actor::facing::FacingIndicator;
use crate::game::actor::faction::Faction;
use crate::game::actor::movement::input::movement_action;
use crate::game::GameLayer;
use crate::game::GameRoot;
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

pub fn player(entity: EntityWorldMut) {
    let parent = entity.world().resource::<GameRoot>().players;

    actor_helper(entity, None)
        .insert((
            IsPlayer,
            Faction::Player,
            CollisionLayers::new(GameLayer::Player, LayerMask::ALL),
            FaceCursor,
            // TODO: This is for testing hit effects until we get actual projectiles / attacks.
            crate::game::combat::hit::Hitbox,
            crate::game::combat::damage::HitboxDamage(2.0),
            crate::game::combat::knockback::HitboxKnockback { force: 150.0 },
        ))
        // TODO: This is for testing movement until it's card-controlled.
        .add(movement_action)
        // TODO: This is for testing attack until it's card-controlled.
        .add(attack_action)
        .set_parent(parent)
        .with_children(|children| {
            children
                .spawn_with(FacingIndicator {
                    distance: vec2(6.0, 5.0),
                })
                .insert(Transform::from_translation(vec3(0.0, -0.5, 2.0)));
        });
}
