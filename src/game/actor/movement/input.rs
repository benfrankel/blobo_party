use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::core::UpdateSet;
use crate::game::actor::movement::MovementController;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<MovementAction>();
}

#[derive(Actionlike, Eq, PartialEq, Hash, Copy, Clone, Reflect)]
pub enum MovementAction {
    Move,
}

impl Configure for MovementAction {
    fn configure(app: &mut App) {
        app.add_plugins(InputManagerPlugin::<Self>::default());
        app.add_systems(Update, apply_movement_action.in_set(UpdateSet::RecordInput));
    }
}

fn apply_movement_action(
    mut movement_query: Query<(&ActionState<MovementAction>, &mut MovementController)>,
) {
    for (action, mut controller) in &mut movement_query {
        let input = c!(action.axis_pair(&MovementAction::Move));
        controller.0 = input.xy().clamp_length_max(1.0);
    }
}
