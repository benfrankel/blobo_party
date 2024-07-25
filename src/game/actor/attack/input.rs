use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::core::UpdateSet;
use crate::game::actor::attack::AttackController;
use crate::game::actor::facing::Facing;
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
        app.add_systems(Update, record_attack_action.in_set(UpdateSet::RecordInput));
    }
}

fn record_attack_action(
    mut action_query: Query<(
        &ActionState<AttackAction>,
        Option<&Facing>,
        &mut AttackController,
    )>,
) {
    for (action, facing, mut controller) in &mut action_query {
        controller.aim += action
            .axis_pair(&AttackAction::Aim)
            .filter(|x| x.xy() != Vec2::ZERO)
            .map(|x| x.xy().clamp_length_max(1.0))
            .or_else(|| facing.map(|x| x.0.as_vec2()))
            .unwrap_or_default();
        controller.fire = action.just_pressed(&AttackAction::Fire);
    }
}

pub fn attack_action(mut entity: EntityWorldMut) {
    entity.insert(InputManagerBundle::with_map(
        InputMap::default()
            .insert(AttackAction::Aim, DualAxis::right_stick())
            .insert(AttackAction::Aim, VirtualDPad::arrow_keys())
            .insert(AttackAction::Fire, MouseButton::Left)
            .insert(AttackAction::Fire, KeyCode::Space)
            .build(),
    ));
}
