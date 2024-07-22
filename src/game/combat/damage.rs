use bevy::prelude::*;

use crate::game::combat::hit::OnHit;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<HitboxDamage>();
}

#[derive(Event)]
pub struct OnDamage(pub f32);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct HitboxDamage(pub f32);

impl Configure for HitboxDamage {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.observe(apply_hitbox_damage);
    }
}

fn apply_hitbox_damage(
    trigger: Trigger<OnHit>,
    mut commands: Commands,
    damage_query: Query<&HitboxDamage>,
) {
    let &OnHit(hitbox, hurtbox) = trigger.event();
    let damage = r!(damage_query.get(hitbox));
    commands.entity(hurtbox).trigger(OnDamage(damage.0));
}
