use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::core::UpdateSet;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(Movement, MovementController, MovementAction)>();
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Movement {
    pub accel: f32,
    pub brake_decel: f32,
    pub max_speed: f32,
}

impl Configure for Movement {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(Update, apply_movement.in_set(UpdateSet::Update));
    }
}

const EPSILON: f32 = 0.01;

fn apply_movement(
    time: Res<Time>,
    mut movement_query: Query<(&Movement, &MovementController, &mut LinearVelocity)>,
) {
    let dt = time.delta_seconds();

    for (movement, controller, mut velocity) in &mut movement_query {
        if controller.0 != Vec2::ZERO {
            velocity.0 += movement.accel * controller.0 * dt;
            velocity.0 = velocity.0.clamp_length_max(movement.max_speed);
        } else if velocity.0.length_squared() > EPSILON {
            velocity.0 *= movement.brake_decel.powf(dt);
        } else {
            velocity.0 = Vec2::ZERO;
        }
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct MovementController(pub Vec2);

impl Configure for MovementController {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

#[derive(Actionlike, Eq, PartialEq, Hash, Copy, Clone, Reflect)]
pub enum MovementAction {
    Move,
}

impl Configure for MovementAction {
    fn configure(app: &mut App) {
        app.add_plugins(InputManagerPlugin::<Self>::default());
        app.add_systems(Update, record_movement_input.in_set(UpdateSet::RecordInput));
    }
}

fn record_movement_input(
    mut movement_query: Query<(&ActionState<MovementAction>, &mut MovementController)>,
) {
    for (action, mut controller) in &mut movement_query {
        let input = c!(action.axis_pair(&MovementAction::Move));
        controller.0 = input.xy().clamp_length_max(1.0);
    }
}
