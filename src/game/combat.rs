use avian2d::prelude::*;
use bevy::prelude::*;

use crate::core::UpdateSet;
use crate::game::actor::health::Health;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(Hitbox, Hurtbox, HitEvent, HitDamage)>();
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
pub struct Hurtbox;

impl Configure for Hurtbox {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

#[derive(Event)]
pub struct HitEvent(pub Entity, pub Entity);

impl Configure for HitEvent {
    fn configure(app: &mut App) {
        app.add_event::<HitEvent>();
        app.add_systems(Update, detect_hit_event.in_set(UpdateSet::SyncEarly));
    }
}

fn detect_hit_event(
    mut collision_events: EventReader<CollisionStarted>,
    mut hit_events: EventWriter<HitEvent>,
    hitbox_query: Query<(), With<Hitbox>>,
    hurtbox_query: Query<(), With<Hurtbox>>,
) {
    for &CollisionStarted(a, b) in collision_events.read() {
        for (a, b) in [(a, b), (b, a)] {
            if hitbox_query.contains(a) && hurtbox_query.contains(b) {
                hit_events.send(HitEvent(a, b));
            }
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct HitDamage(pub f32);

impl Configure for HitDamage {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            apply_hit_damage
                .in_set(UpdateSet::Update)
                .run_if(on_event::<HitEvent>()),
        );
    }
}

fn apply_hit_damage(
    mut hit_events: EventReader<HitEvent>,
    hitbox_query: Query<&HitDamage>,
    mut hurtbox_query: Query<&mut Health>,
) {
    for &HitEvent(a, b) in hit_events.read() {
        let damage = c!(hitbox_query.get(a));
        let mut health = c!(hurtbox_query.get_mut(b));

        health.current -= damage.0;
    }
}

// TODO: HitKnockback
