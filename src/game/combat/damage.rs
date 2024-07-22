use bevy::prelude::*;

use crate::game::actor::health::Health;
use crate::game::combat::collision::OnHit;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<HitboxDamage>();
}

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
    damage_query: Query<&HitboxDamage>,
    mut health_query: Query<&mut Health>,
) {
    let &OnHit(hitbox, hurtbox) = trigger.event();
    let damage = r!(damage_query.get(hitbox));
    let mut health = r!(health_query.get_mut(hurtbox));

    health.current -= damage.0;
}
