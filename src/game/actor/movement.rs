pub mod input;

use avian2d::prelude::*;
use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::core::UpdateSet;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(Movement, MovementController)>();

    app.add_plugins(input::plugin);
}

/// Movement parameters.
#[derive(Component, Reflect, Serialize, Deserialize, Copy, Clone)]
#[reflect(Component)]
pub struct Movement {
    /// The acceleration rate (pixels per second^2).
    /// Applies when the controller is active and speed is under control.
    pub accel: f32,
    /// The deceleration factor (multiplier per second).
    /// Applies when the controller is inactive or speed is out of control.
    pub decel: f32,
    /// The max "under control" speed (pixels per second).
    pub speed: f32,
}

impl Configure for Movement {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(Update, apply_movement.in_set(UpdateSet::Update));
    }
}

fn apply_movement(
    time: Res<Time>,
    mut movement_query: Query<(&Movement, &MovementController, &mut LinearVelocity)>,
) {
    let dt = time.delta_seconds();

    for (movement, controller, mut velocity) in &mut movement_query {
        if controller.0 == Vec2::ZERO || velocity.0.length_squared() >= movement.speed.powi(2) {
            // Apply deceleration.
            velocity.0 *= movement.decel.powf(dt);
        } else {
            // Apply acceleration.
            velocity.0 += movement.accel * controller.0 * dt;
            velocity.0 = velocity.0.clamp_length_max(movement.speed);
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
