pub mod up;
pub mod xp;

use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(ConfigHandle<LevelConfig>, PlayerLevel)>();

    app.add_plugins((up::plugin, xp::plugin));
}

#[derive(Asset, Reflect, Serialize, Deserialize)]
pub struct LevelConfig {
    /// The level sequence (final level repeats forever).
    pub levels: Vec<Level>,
}

impl Config for LevelConfig {
    const PATH: &'static str = "config/level.ron";
    const EXTENSION: &'static str = "level.ron";
}

impl LevelConfig {
    pub fn level(&self, idx: usize) -> &Level {
        &self.levels[idx.min(self.levels.len() - 1)]
    }
}

#[derive(Reflect, Serialize, Deserialize)]
pub struct Level {
    /// The XP cost to level up from this level.
    pub xp_cost: f32,
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct PlayerLevel {
    /// The current level.
    pub current: usize,
    /// The number of pending level-ups.
    pub up: usize,
}

impl Configure for PlayerLevel {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_resource::<Self>();
    }
}
