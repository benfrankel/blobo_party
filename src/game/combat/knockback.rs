use avian2d::prelude::*;
use bevy::prelude::*;

use crate::game::combat::hit::OnHit;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<HitboxKnockback>();
}

/// Scales with projectile speed.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct HitboxKnockback(pub f32, pub bool);

impl Configure for HitboxKnockback {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.observe(apply_hitbox_knockback);
    }
}

fn apply_hitbox_knockback(
    trigger: Trigger<OnHit>,
    knockback_query: Query<&HitboxKnockback>,
    mut velocity_query: Query<&mut LinearVelocity>,
    gt_query: Query<&GlobalTransform>,
) {
    let &OnHit(hitbox, hurtbox) = trigger.event();
    let knockback = r!(knockback_query.get(hitbox));

    let knockback = if knockback.1 {
        let hitbox_velocity = r!(velocity_query.get(hitbox)).0;
        hitbox_velocity * knockback.0
    } else {
        let hitbox_pos = r!(gt_query.get(hitbox)).translation().xy();
        let hurtbox_pos = r!(gt_query.get(hurtbox)).translation().xy();
        let direction = r!(Dir2::new(hurtbox_pos - hitbox_pos));
        direction * knockback.0
    };

    let mut velocity = r!(velocity_query.get_mut(hurtbox));
    velocity.0 += knockback;
}
