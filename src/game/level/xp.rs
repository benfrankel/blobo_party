use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::core::UpdateSet;
use crate::game::actor::faction::Faction;
use crate::game::combat::death::OnDeath;
use crate::game::level::Level;
use crate::game::level::LevelConfig;
use crate::ui::prelude::*;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(Xp, OnReceiveXp, XpReward, IsXpBarFill)>();
}

/// The player's XP relative to the current level.
#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct Xp(pub f32);

impl Configure for Xp {
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

fn receive_xp(trigger: Trigger<OnReceiveXp>, mut xp: ResMut<Xp>) {
    xp.0 += trigger.event().0;
}

/// Experience rewarded to the player on death.
#[derive(Component, Reflect, Serialize, Deserialize, Copy, Clone)]
#[reflect(Component)]
#[serde(transparent)]
pub struct XpReward(pub f32);

impl Configure for XpReward {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.observe(apply_xp_reward);
    }
}

impl Default for XpReward {
    fn default() -> Self {
        Self(10.0)
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

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct IsXpBarFill;

impl Configure for IsXpBarFill {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(Update, update_xp_bar_fill.in_set(UpdateSet::SyncLate));
    }
}

fn update_xp_bar_fill(
    config: ConfigRef<LevelConfig>,
    level: Res<Level>,
    xp: Res<Xp>,
    mut xp_bar_fill_query: Query<&mut Style, With<IsXpBarFill>>,
) {
    let config = r!(config.get());
    if config.levels.is_empty() {
        return;
    }
    let xp_cost = config.level(level.current + level.up).xp_cost;
    let width = Percent(xp.0 / xp_cost * 100.0);

    for mut style in &mut xp_bar_fill_query {
        style.width = width;
    }
}
