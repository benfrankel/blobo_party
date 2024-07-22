use bevy::math::vec2;
use bevy::math::vec3;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use super::movement::MovementAction;
use crate::game::actor::actor_helper;
use crate::game::actor::facing::FaceCursor;
use crate::game::actor::facing::FacingIndicator;
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
        .insert((
            IsPlayer,
            FaceCursor,
            InputManagerBundle::with_map(
                InputMap::default()
                    .insert(MovementAction::Move, VirtualDPad::wasd())
                    .insert(MovementAction::Move, VirtualDPad::arrow_keys())
                    .insert(MovementAction::Move, DualAxis::left_stick())
                    .build(),
            ),
        ))
        .with_children(|children| {
            children
                .spawn_with(FacingIndicator {
                    radius: vec2(6.0, 5.0),
                })
                .insert(Transform::from_translation(vec3(0.0, -0.5, 2.0)));
        });
}
