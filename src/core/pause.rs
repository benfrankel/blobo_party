use bevy::prelude::*;
use pyri_state::prelude::*;

use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<Pause>();
}

#[derive(State, Eq, PartialEq, Clone, Reflect, Default)]
#[reflect(Resource)]
pub struct Pause;

impl Configure for Pause {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_state::<Self>();
    }
}
