use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use pyri_state::prelude::*;

use crate::core::pause::Pause;
use crate::core::UpdateSet;
use crate::game::actor::movement::MovementController;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<MovementAction>();
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Reflect, Debug)]
enum MovementAction {
    Move,
}

impl Actionlike for MovementAction {
    fn input_control_kind(&self) -> InputControlKind {
        match self {
            Self::Move => InputControlKind::DualAxis,
        }
    }
}

impl Configure for MovementAction {
    fn configure(app: &mut App) {
        app.add_plugins(InputManagerPlugin::<Self>::default());
        app.add_systems(
            Update,
            record_movement_action
                .in_set(UpdateSet::RecordInput)
                .run_if(Pause::is_disabled),
        );
    }
}

fn record_movement_action(
    mut action_query: Query<(&ActionState<MovementAction>, &mut MovementController)>,
) {
    for (action, mut controller) in &mut action_query {
        controller.0 += action
            .axis_pair(&MovementAction::Move)
            .xy()
            .clamp_length_max(1.0);
    }
}

pub fn movement_action(mut entity: EntityWorldMut) {
    entity.insert(InputManagerBundle::with_map(
        InputMap::default()
            .with_dual_axis(MovementAction::Move, GamepadStick::LEFT)
            .with_dual_axis(MovementAction::Move, KeyboardVirtualDPad::WASD),
    ));
}
