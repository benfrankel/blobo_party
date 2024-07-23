use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::core::UpdateSet;
use crate::game::actor::faction::Faction;
use crate::game::combat::death::OnDeath;
use crate::util::prelude::*;

// TODO: System that enters level up menu on LevelUp event.
// TODO: XpBar that updates based on (xp, xp_cost)
pub(super) fn plugin(app: &mut App) {
    app.configure::<(
        ConfigHandle<LevelConfig>,
        PlayerLevel,
        PlayerLevelUp,
        PlayerXp,
        OnReceiveXp,
        XpReward,
    )>();
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
        app.add_systems(
            Update,
            update_player_level_from_xp.in_set(UpdateSet::TriggerLevelUp),
        );
    }
}

fn update_player_level_from_xp(
    config: ConfigRef<LevelConfig>,
    mut player_level: ResMut<PlayerLevel>,
    mut player_xp: ResMut<PlayerXp>,
) {
    let config = r!(config.get());
    if config.levels.is_empty() {
        return;
    }

    loop {
        let level = config.level(player_level.current + player_level.up);
        if player_xp.0 < level.xp_cost {
            break;
        }

        player_xp.0 -= level.xp_cost;
        player_level.up += 1;
    }
}

/// A buffered event sent when the player levels up.
#[derive(Event)]
pub struct PlayerLevelUp;

impl Configure for PlayerLevelUp {
    fn configure(app: &mut App) {
        app.add_event::<Self>();
        // TODO: Run if not in level up menu.
        app.add_systems(
            Update,
            trigger_player_level_up
                .in_set(UpdateSet::TriggerLevelUp)
                .after(update_player_level_from_xp),
        );
    }
}

fn trigger_player_level_up(
    mut level_up_events: EventWriter<PlayerLevelUp>,
    mut level: ResMut<PlayerLevel>,
) {
    if level.up > 0 {
        level.up -= 1;
        level.current += 1;
        level_up_events.send(PlayerLevelUp);
    }
}

/// The player's XP relative to the current level.
#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct PlayerXp(pub f32);

impl Configure for PlayerXp {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_resource::<Self>();
    }
}

/// An observable event triggered when the player receives XP.
#[derive(Event)]
pub struct OnReceiveXp(pub f32);

impl Configure for OnReceiveXp {
    fn configure(app: &mut App) {
        app.add_event::<Self>();
        app.observe(receive_xp);
    }
}

fn receive_xp(trigger: Trigger<OnReceiveXp>, mut xp: ResMut<PlayerXp>) {
    xp.0 += trigger.event().0;
}

/// Experience rewarded to the player on death.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct XpReward(pub f32);

impl Configure for XpReward {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.observe(apply_xp_reward);
    }
}

fn apply_xp_reward(
    trigger: Trigger<OnDeath>,
    mut commands: Commands,
    death_query: Query<(&Faction, &XpReward)>,
) {
    let entity = r!(trigger.get_entity());
    let (faction, reward) = r!(death_query.get(entity));

    if faction.is_enemy() {
        commands.trigger(OnReceiveXp(reward.0));
    }
}
