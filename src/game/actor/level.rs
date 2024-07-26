pub mod up;
pub mod xp;

use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::core::UpdateSet;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(ConfigHandle<LevelConfig>, Level, IsLevelIndicator)>();

    app.add_plugins((up::plugin, xp::plugin));
}

#[derive(Asset, Reflect, Serialize, Deserialize)]
pub struct LevelConfig {
    /// The level sequence (final level repeats forever).
    pub levels: Vec<LevelData>,
}

impl Config for LevelConfig {
    const PATH: &'static str = "config/level.ron";
    const EXTENSION: &'static str = "level.ron";
}

impl LevelConfig {
    pub fn level(&self, idx: usize) -> &LevelData {
        &self.levels[idx.min(self.levels.len() - 1)]
    }
}

#[derive(Reflect, Serialize, Deserialize)]
pub struct LevelData {
    /// The XP cost to level up from this level.
    pub xp_cost: f32,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Level {
    /// The current level.
    pub current: usize,
    /// The number of pending level-ups.
    pub up: usize,
}

impl Configure for Level {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct IsLevelIndicator;

impl Configure for IsLevelIndicator {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(Update, update_level_indicator.in_set(UpdateSet::SyncLate));
    }
}

fn update_level_indicator(
    mut indicator_query: Query<(&mut Text, &Selection), With<IsLevelIndicator>>,
    level_query: Query<&Level>,
) {
    for (mut text, selection) in &mut indicator_query {
        let level = c!(level_query.get(selection.0));
        let level = level.current + level.up;
        let level = level.to_string();

        for section in &mut text.sections {
            section.value.clone_from(&level);
        }
    }
}
