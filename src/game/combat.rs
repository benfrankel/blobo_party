pub mod damage;
pub mod death;
pub mod hit;
// TODO: pub mod knockback

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((hit::plugin, damage::plugin, death::plugin));
}
