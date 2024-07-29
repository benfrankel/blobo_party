pub mod input;
pub mod smoke;

use avian2d::prelude::*;
use bevy::prelude::*;
use pyri_state::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::core::pause::Pause;
use crate::core::UpdateSet;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(
        Movement,
        MovementController,
        OldMovementController,
        MovementEvent,
    )>();

    app.add_plugins((input::plugin, smoke::plugin));
}

/// Movement parameters.
#[derive(Component, Reflect, Serialize, Deserialize, Copy, Clone)]
#[reflect(Component)]
#[serde(default)]
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
        app.add_systems(
            Update,
            apply_movement
                .in_set(UpdateSet::Update)
                .run_if(Pause::is_disabled),
        );
    }
}

impl Default for Movement {
    fn default() -> Self {
        Self {
            accel: 1000.0,
            decel: 0.0001,
            speed: 80.0,
        }
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
        app.add_systems(
            Update,
            reset_movement_controller
                .in_set(UpdateSet::SyncEarly)
                .run_if(Pause::is_disabled),
        );
    }
}

fn reset_movement_controller(mut controller_query: Query<&mut MovementController>) {
    for mut controller in &mut controller_query {
        controller.0 = Vec2::ZERO;
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct OldMovementController(pub Vec2);

impl Configure for OldMovementController {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            sync_old_movement_controller
                .in_set(UpdateSet::SyncLate)
                .run_if(Pause::is_disabled),
        );
    }
}

fn sync_old_movement_controller(
    mut controller_query: Query<(&mut OldMovementController, &MovementController)>,
) {
    for (mut old, new) in &mut controller_query {
        old.0 = new.0;
    }
}

/// A buffered event sent when an actor's movement changes significantly.
#[allow(dead_code)]
#[derive(Event)]
pub enum MovementEvent {
    Start(Entity),
    Stop(Entity),
    Reverse(Entity),
}

impl Configure for MovementEvent {
    fn configure(app: &mut App) {
        app.add_event::<Self>();
        app.add_systems(Update, detect_movement_event.in_set(UpdateSet::Update));
    }
}

fn detect_movement_event(
    mut movement_events: EventWriter<MovementEvent>,
    controller_query: Query<(Entity, &OldMovementController, &MovementController)>,
) {
    for (entity, old, new) in &controller_query {
        movement_events.send(match (old.0, new.0) {
            (Vec2::ZERO, y) if y != Vec2::ZERO => MovementEvent::Start(entity),
            (x, Vec2::ZERO) if x != Vec2::ZERO => MovementEvent::Stop(entity),
            (x, y) if x.dot(y) < 0.0 => MovementEvent::Reverse(entity),
            _ => continue,
        });
    }
}
