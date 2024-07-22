pub mod collision;
pub mod damage;
// TODO: pub mod knockback

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((collision::plugin, damage::plugin));
}
