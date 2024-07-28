use bevy::prelude::*;

use crate::core::UpdateSet;
use crate::game::actor::attack::Attack;
use crate::game::actor::attack::AttackController;
use crate::game::actor::facing::Facing;
use crate::game::audio::music::on_beat;
use crate::game::cleanup::RemoveOnBeat;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(AimTowardsFacing, AttackOnBeat)>();
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct AimTowardsFacing;

impl Configure for AimTowardsFacing {
    fn configure(app: &mut App) {
        app.configure::<RemoveOnBeat<Self>>();
        app.register_type::<Self>();
        app.add_systems(
            Update,
            apply_aim_towards_facing.in_set(UpdateSet::RecordInput),
        );
    }
}

fn apply_aim_towards_facing(
    mut attack_query: Query<(&mut AttackController, &Facing), With<AimTowardsFacing>>,
) {
    for (mut controller, facing) in &mut attack_query {
        controller.aim += *facing.0;
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct AttackOnBeat(pub Attack);

impl Configure for AttackOnBeat {
    fn configure(app: &mut App) {
        app.configure::<RemoveOnBeat<Self>>();
        app.register_type::<Self>();
        app.add_systems(
            Update,
            attack_on_beat
                .in_set(UpdateSet::RecordInput)
                .run_if(on_beat(4)),
        );
    }
}

fn attack_on_beat(mut attack_query: Query<(&mut Attack, &mut AttackController, &AttackOnBeat)>) {
    for (mut attack, mut controller, attack_on_beat) in &mut attack_query {
        attack.power = attack_on_beat.0.power;
        attack.force = attack_on_beat.0.force;
        attack.projectile_key = attack_on_beat.0.projectile_key.clone();
        attack.multi_shot = attack_on_beat.0.multi_shot.clone();
        attack.child_projectile = attack_on_beat.0.child_projectile.clone();

        controller.fire = true;
    }
}
