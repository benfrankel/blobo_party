use bevy::math::vec2;
use bevy::math::vec3;
use bevy::prelude::*;

use crate::game::actor::actor_helper;
use crate::game::actor::facing::FaceCursor;
use crate::game::actor::facing::FacingIndicator;
use crate::game::deck_dock::DrawDeck;
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
    actor_helper(entity, None)
        .insert((IsPlayer, DrawDeck, FaceCursor))
        .with_children(|children| {
            children
                .spawn_with(FacingIndicator {
                    radius: vec2(6.0, 5.0),
                })
                .insert(Transform::from_translation(vec3(0.0, -0.5, 2.0)));
        });
}
