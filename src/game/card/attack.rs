use bevy::prelude::*;

use crate::core::UpdateSet;
use crate::game::actor::attack::Attack;
use crate::game::actor::attack::AttackController;
use crate::game::actor::facing::Facing;
use crate::game::cleanup::RemoveOnBeat;
use crate::game::music::beat::on_beat;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(AimTowardsFacing, DoubleBeat)>();
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
pub struct DoubleBeat;

impl Configure for DoubleBeat {
    fn configure(app: &mut App) {
        app.configure::<RemoveOnBeat<Self>>();
        app.register_type::<Self>();
        app.add_systems(
            Update,
            double_beat
                .in_set(UpdateSet::RecordInput)
                .run_if(on_beat(1)),
        );
    }
}

fn double_beat(mut attack_query: Query<(&mut Attack, &mut AttackController), With<DoubleBeat>>) {
    for (mut attack, mut controller) in &mut attack_query {
        // TODO: Put these values in the config somehow.
        attack.power = 2.0;
        attack.force = 4.0;
        attack.projectile = Some("quarter_note".to_string());

        controller.fire = true;
    }
}
