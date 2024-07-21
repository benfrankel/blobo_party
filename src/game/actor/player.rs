use bevy::prelude::*;

use crate::game::actor::actor_helper;
use crate::game::actor::facing::FaceCursor;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<IsPlayer>();
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct IsPlayer;

impl Configure for IsPlayer {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

pub fn player(entity: EntityWorldMut) {
    actor_helper(entity, None).insert((IsPlayer, FaceCursor));
}
