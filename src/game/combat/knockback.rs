use avian2d::prelude::*;
use bevy::prelude::*;

use crate::game::combat::hit::OnHit;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<HitboxKnockback>();
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct HitboxKnockback {
    pub force: f32,
}

impl Configure for HitboxKnockback {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.observe(apply_hitbox_knockback);
    }
}

fn apply_hitbox_knockback(
    trigger: Trigger<OnHit>,
    hitbox_query: Query<(&GlobalTransform, &HitboxKnockback)>,
    mut hurtbox_query: Query<(&GlobalTransform, &mut LinearVelocity)>,
) {
    let &OnHit(hitbox, hurtbox) = trigger.event();
    let (hitbox_gt, knockback) = r!(hitbox_query.get(hitbox));
    let (hurtbox_gt, mut velocity) = r!(hurtbox_query.get_mut(hurtbox));

    let hitbox_pos = hitbox_gt.translation().xy();
    let hurtbox_pos = hurtbox_gt.translation().xy();
    let direction = Dir2::new(hurtbox_pos - hitbox_pos).unwrap_or(Dir2::EAST);
    velocity.0 += knockback.force * direction;
}
