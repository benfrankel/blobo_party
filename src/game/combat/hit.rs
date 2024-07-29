use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use rand::Rng as _;

use crate::core::UpdateSet;
use crate::game::cleanup::RemoveOnTimer;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(Hitbox, Hurtbox, OnHit, HurtSfx, Immune)>();
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Hitbox;

impl Configure for Hitbox {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Immune;

impl Configure for Immune {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.configure::<RemoveOnTimer<Self>>();
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Hurtbox;

impl Configure for Hurtbox {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

/// An observable event for when a hitbox hits a hurtbox.
#[derive(Event)]
pub struct OnHit(pub Entity, pub Entity);

impl Configure for OnHit {
    fn configure(app: &mut App) {
        app.add_systems(Update, trigger_hit.in_set(UpdateSet::Update));
    }
}

fn trigger_hit(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionStarted>,
    hitbox_query: Query<(), With<Hitbox>>,
    hurtbox_query: Query<(), (With<Hurtbox>, Without<Immune>)>,
) {
    for &CollisionStarted(a, b) in collision_events.read() {
        for (a, b) in [(a, b), (b, a)] {
            if hitbox_query.contains(a) && hurtbox_query.contains(b) {
                commands.trigger(OnHit(a, b));
            }
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct HurtSfx(pub Handle<AudioSource>, pub f64);

impl Configure for HurtSfx {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.observe(play_hurt_sfx);
    }
}

fn play_hurt_sfx(trigger: Trigger<OnHit>, sfx_query: Query<&HurtSfx>, audio: Res<Audio>) {
    let sfx = r!(sfx_query.get(trigger.event().1));
    audio
        .play(sfx.0.clone())
        .with_volume(sfx.1)
        .with_playback_rate(rand::thread_rng().gen_range(0.7..1.6));
}
