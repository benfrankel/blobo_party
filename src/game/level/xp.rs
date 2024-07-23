use bevy::prelude::*;

use crate::game::actor::faction::Faction;
use crate::game::combat::death::OnDeath;
use crate::util::prelude::*;

// TODO: XpBar that updates based on (xp, xp_cost)
pub(super) fn plugin(app: &mut App) {
    app.configure::<(PlayerXp, OnReceiveXp, XpReward)>();
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
