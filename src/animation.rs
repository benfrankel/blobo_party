pub mod backup;
pub mod offset;
pub mod transition;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((backup::plugin, offset::plugin, transition::plugin));
}
