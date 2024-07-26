use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use pyri_state::prelude::*;

use crate::core::pause::Pause;
use crate::core::UpdateSet;
use crate::game::actor::attack::AttackController;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<AttackAction>();
}

#[derive(Actionlike, Eq, PartialEq, Hash, Copy, Clone, Reflect)]
enum AttackAction {
    Aim,
    Fire,
}

impl Configure for AttackAction {
    fn configure(app: &mut App) {
        app.add_plugins(InputManagerPlugin::<Self>::default());
        app.add_systems(
            Update,
            record_attack_action
                .in_set(UpdateSet::RecordInput)
                .run_if(Pause::is_disabled),
        );
    }
}

fn record_attack_action(
    mut action_query: Query<(&ActionState<AttackAction>, &mut AttackController)>,
) {
    for (action, mut controller) in &mut action_query {
        controller.aim += cq!(action.axis_pair(&AttackAction::Aim))
            .xy()
            .clamp_length_max(1.0);
        controller.fire |= action.just_pressed(&AttackAction::Fire);
    }
}

pub fn attack_action(mut entity: EntityWorldMut) {
    entity.insert(InputManagerBundle::with_map(
        InputMap::default()
            .insert(AttackAction::Aim, DualAxis::right_stick())
            .insert(AttackAction::Aim, VirtualDPad::arrow_keys())
            .insert(AttackAction::Fire, GamepadButtonType::East)
            .insert(AttackAction::Fire, VirtualDPad::arrow_keys())
            .build(),
    ));
}
