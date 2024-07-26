use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::core::UpdateSet;
use crate::game::actor::faction::Faction;
use crate::game::actor::level::Level;
use crate::game::actor::level::LevelConfig;
use crate::game::actor::player::IsPlayer;
use crate::game::combat::death::OnDeath;
use crate::ui::prelude::*;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(Xp, OnXpReward, XpReward, IsXpBarFill)>();
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Xp {
    /// The total amount of XP.
    pub total: f32,
    /// The amount of XP relative to the current level.
    pub relative: f32,
}

impl Configure for Xp {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

impl Xp {
    pub fn gain(&mut self, amount: f32) {
        self.total += amount;
        self.relative += amount;
    }
}

/// An observable event triggered when an entity receives XP.
#[derive(Event)]
pub struct OnXpReward(pub f32);

impl Configure for OnXpReward {
    fn configure(app: &mut App) {
        app.add_event::<Self>();
        app.observe(receive_xp);
    }
}

// TODO: Not needed for this jam game, but it would be "more correct" to track
//       the owner of the projectile that killed the actor with the `XpReward`,
//       and only trigger `OnXpReward` for that entity.
fn receive_xp(trigger: Trigger<OnXpReward>, mut xp_query: Query<&mut Xp, With<IsPlayer>>) {
    for mut xp in &mut xp_query {
        xp.gain(trigger.event().0);
    }
}

/// Experience points rewarded to the killer on death.
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
        commands.trigger(OnXpReward(reward.0));
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
    level_query: Query<(&Level, &Xp)>,
    mut xp_bar_fill_query: Query<(&mut Style, &Selection), With<IsXpBarFill>>,
) {
    let config = r!(config.get());
    if config.levels.is_empty() {
        return;
    }

    for (mut style, selection) in &mut xp_bar_fill_query {
        let (level, xp) = r!(level_query.get(selection.0));
        let level_cost = config.level(level.current + level.up).xp_cost;

        style.width = Percent(xp.relative / level_cost * 100.0);
    }
}
