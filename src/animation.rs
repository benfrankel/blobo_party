pub mod backup;
pub mod offset;
pub mod transition;

use bevy::prelude::*;
use bevy_tweening::*;
use pyri_state::prelude::*;

use crate::core::pause::Pause;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(TweeningPlugin);
    app.add_systems(
        StateFlush,
        Pause.on_edge(unpause_tweens::<Sprite>, pause_tweens::<Sprite>),
    );

    app.add_plugins((backup::plugin, offset::plugin, transition::plugin));
}

fn unpause_tweens<C: Component>(mut tween_query: Query<&mut Animator<C>>) {
    for mut tween in &mut tween_query {
        tween.state = AnimatorState::Playing;
    }
}

fn pause_tweens<C: Component>(mut tween_query: Query<&mut Animator<C>>) {
    for mut tween in &mut tween_query {
        tween.state = AnimatorState::Paused;
    }
}
