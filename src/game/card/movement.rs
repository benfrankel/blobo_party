use bevy::prelude::*;

use crate::core::UpdateSet;
use crate::game::actor::facing::Facing;
use crate::game::actor::movement::MovementController;
use crate::game::cleanup::RemoveOnBeat;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<MoveTowardsFacing>();
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct MoveTowardsFacing;

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
    mut movement_query: Query<(&mut MovementController, &Facing), With<MoveTowardsFacing>>,
) {
    for (mut controller, facing) in &mut movement_query {
        controller.0 += *facing.0;
    }
}
