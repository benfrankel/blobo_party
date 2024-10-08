use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use rand::Rng as _;

use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(IsDead, DespawnOnDeath, DeathSfx)>();
}

/// An observable event triggered when an actor dies.
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

fn despawn_on_death(
    trigger: Trigger<OnDeath>,
    despawn_query: Query<(), With<DespawnOnDeath>>,
    mut despawn: ResMut<LateDespawn>,
) {
    let entity = r!(trigger.get_entity());
    if despawn_query.contains(entity) {
        despawn.recursive(entity);
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct DeathSfx(pub Handle<AudioSource>, pub f64);

impl Configure for DeathSfx {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.observe(play_death_sfx);
    }
}

fn play_death_sfx(trigger: Trigger<OnDeath>, sfx_query: Query<&DeathSfx>, audio: Res<Audio>) {
    let entity = r!(trigger.get_entity());
    let sfx = r!(sfx_query.get(entity));
    audio
        .play(sfx.0.clone())
        .with_volume(sfx.1)
        .with_playback_rate(rand::thread_rng().gen_range(0.8..1.4));
}
