use bevy::prelude::*;

use crate::core::UpdateSet;
use crate::game::actor::facing::Facing;
use crate::game::actor::movement::MovementController;
use crate::game::card::remove::*;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<RemoveOnBeat<Move>>();

    app.add_systems(
        Update,
        handle_move.in_set(UpdateSet::RecordInput), // TODO: Is this the best choice?
    );
}

#[derive(Component, Reflect)]
pub struct Move;

fn handle_move(mut moves: Query<(&Facing, &mut MovementController), With<Move>>) {
    for (facing, mut controller) in &mut moves {
        controller.0 += *facing.0;
    }
}
