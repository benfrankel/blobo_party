// Disable console on windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::prelude::*;

fn main() -> AppExit {
    App::new().add_plugins(bevy_jam_5::plugin).run()
}
