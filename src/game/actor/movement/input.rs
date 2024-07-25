use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::core::UpdateSet;
use crate::game::actor::movement::MovementController;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<MovementAction>();
}

#[derive(Actionlike, Eq, PartialEq, Hash, Copy, Clone, Reflect)]
enum MovementAction {
    Move,
}

impl Configure for MovementAction {
    fn configure(app: &mut App) {
        app.add_plugins(InputManagerPlugin::<Self>::default());
        app.add_systems(
            Update,
            record_movement_action.in_set(UpdateSet::RecordInput),
        );
    }
}

fn record_movement_action(
    mut action_query: Query<(&ActionState<MovementAction>, &mut MovementController)>,
) {
    for (action, mut controller) in &mut action_query {
        let input = cq!(action.axis_pair(&MovementAction::Move));
        controller.0 = input.xy().clamp_length_max(1.0);
    }
}

pub fn movement_action(mut entity: EntityWorldMut) {
    entity.insert(InputManagerBundle::with_map(
        InputMap::default()
            .insert(MovementAction::Move, DualAxis::left_stick())
            .insert(MovementAction::Move, VirtualDPad::wasd())
            .build(),
    ));
}
