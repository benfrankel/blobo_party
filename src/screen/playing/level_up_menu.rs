// TODO
#![allow(unused)]

use bevy::prelude::*;
use pyri_state::prelude::*;

use crate::core::pause::Pause;
use crate::screen::playing::PlayingMenu;
use crate::ui::prelude::*;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        StateFlush,
        PlayingMenu::LevelUp.on_edge(Pause::disable, (Pause::enable_default, open_level_up_menu)),
    );
}

fn open_level_up_menu(mut commands: Commands) {
    commands.spawn_with(level_up_menu);
}

fn level_up_menu(entity: Entity, world: &mut World) {
    // TODO
}
