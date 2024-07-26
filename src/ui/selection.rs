use bevy::prelude::*;

use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<Selection>();
}

/// The entity that a UI display will pull values from.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Selection(pub Entity);

impl Configure for Selection {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

impl Default for Selection {
    fn default() -> Self {
        Self(Entity::PLACEHOLDER)
    }
}
