use bevy::prelude::*;

use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(IsDead, DespawnOnDeath)>();
}

/// An observable event on an actor's death.
/// Remember to filter out `IsDead` entities before triggering this event.
#[derive(Event)]
pub struct OnDeath;

/// A marker component for dead actors (to help avoid double-death).
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct IsDead;

impl Configure for IsDead {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.observe(is_dead);
    }
}

fn is_dead(trigger: Trigger<OnDeath>, mut commands: Commands) {
    commands.entity(r!(trigger.get_entity())).insert(IsDead);
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct DespawnOnDeath;

impl Configure for DespawnOnDeath {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.observe(despawn_on_death);
    }
}

fn despawn_on_death(trigger: Trigger<OnDeath>, mut despawn: ResMut<DespawnSet>) {
    despawn.recursive(r!(trigger.get_entity()));
}
