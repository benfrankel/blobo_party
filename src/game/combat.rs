pub mod damage;
pub mod death;
pub mod hit;
pub mod knockback;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        damage::plugin,
        death::plugin,
        hit::plugin,
        knockback::plugin,
    ));
}
