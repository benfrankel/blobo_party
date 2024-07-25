use bevy::prelude::*;

use crate::core::UpdateSet;
use crate::game::actor::attack::Attack;
use crate::game::actor::attack::AttackController;
use crate::game::actor::facing::Facing;
use crate::game::cleanup::RemoveOnBeat;
use crate::game::music::beat::on_beat;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<RemoveOnBeat<DoubleBeat>>();

    app.add_systems(
        Update,
        double_beat.in_set(UpdateSet::RecordInput) // TODO: Is this the best choice?
                .run_if(on_beat(1)),
    );
}

#[derive(Component, Reflect)]
pub struct DoubleBeat;

fn double_beat(
    mut attacker: Query<(&Facing, &mut Attack, &mut AttackController), With<DoubleBeat>>,
) {
    for (facing, mut attack, mut controller) in &mut attacker {
        attack.power = 2.0;
        attack.force = 4.0;
        attack.projectile = Some("quarter_note".to_string());

        controller.0 = facing.0.into();
    }
}
