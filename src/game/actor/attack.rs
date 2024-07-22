use bevy::prelude::*;

use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(Attack, AttackController)>();
}

/// Attack parameters.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Attack {
    /// Multiplier for effects like damage and knockback.
    pub strength: f32,
}

impl Configure for Attack {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct AttackController(pub Vec2);

impl Configure for AttackController {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}
