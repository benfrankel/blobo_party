use bevy::prelude::*;

use crate::core::UpdateSet;
use crate::game::combat::hit::Immune;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<IsShield>();
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct IsShield;

impl Configure for IsShield {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(Update, update_shield.in_set(UpdateSet::SyncLate));
    }
}

fn update_shield(
    mut shield_query: Query<(&mut Visibility, &Parent), With<IsShield>>,
    immune_query: Query<(), With<Immune>>,
) {
    for (mut visibility, parent) in &mut shield_query {
        *visibility = if immune_query.contains(parent.get()) {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
}
