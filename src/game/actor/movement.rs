pub mod input;

use avian2d::prelude::*;
use bevy::prelude::*;

use crate::core::UpdateSet;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(Movement, MovementController)>();

    app.add_plugins(input::plugin);
}

/// Movement parameters.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Movement {
    /// The acceleration when controller is active (pixels per second^2).
    pub accel: f32,
    /// The deceleration factor when controller is idle (decay per second).
    pub brake_decel: f32,
    /// The maximum speed (pixels per second).
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
