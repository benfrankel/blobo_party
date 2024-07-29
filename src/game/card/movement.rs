use std::f32::consts::TAU;

use bevy::prelude::*;

use crate::core::UpdateSet;
use crate::game::actor::facing::Facing;
use crate::game::actor::movement::Movement;
use crate::game::actor::movement::MovementController;
use crate::game::cleanup::RemoveOnBeat;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<MoveTowardsFacing>();
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct MoveTowardsFacing(pub Movement);

impl Configure for MoveTowardsFacing {
    fn configure(app: &mut App) {
        app.configure::<RemoveOnBeat<Self>>();
        app.register_type::<Self>();
        app.add_systems(
            Update,
            apply_move_towards_facing.in_set(UpdateSet::RecordInput),
        );
    }
}

fn apply_move_towards_facing(
    mut movement_query: Query<(
        &mut Movement,
        &mut MovementController,
        &Facing,
        &MoveTowardsFacing,
    )>,
) {
    for (mut movement, mut controller, facing, move_towards_facing) in &mut movement_query {
        *movement = move_towards_facing.0;

        let facing_angle = Vec2::from_angle(movement.direction * TAU);
        controller.0 += (*facing.0).rotate(facing_angle);
    }
}
