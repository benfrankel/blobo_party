use bevy::prelude::*;

use crate::core::UpdateSet;
use crate::game::level::xp::PlayerXp;
use crate::game::level::LevelConfig;
use crate::game::level::PlayerLevel;
use crate::util::prelude::*;

// TODO: System that enters level up menu on PlayerLevelUp event.
pub(super) fn plugin(app: &mut App) {
    app.configure::<PlayerLevelUp>();
}

/// A buffered event sent when the player levels up.
#[derive(Event)]
pub struct PlayerLevelUp;

impl Configure for PlayerLevelUp {
    fn configure(app: &mut App) {
        app.add_event::<Self>();
        app.add_systems(
            Update,
            (
                update_level_up_from_xp.in_set(UpdateSet::TriggerLevelUp),
                // TODO: Only run if not in level up menu.
                trigger_level_up.in_set(UpdateSet::TriggerLevelUp),
            )
                .chain(),
        );
    }
}

fn update_level_up_from_xp(
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

fn trigger_level_up(
    mut level_up_events: EventWriter<PlayerLevelUp>,
    mut level: ResMut<PlayerLevel>,
) {
    if level.up > 0 {
        level.up -= 1;
        level.current += 1;
        level_up_events.send(PlayerLevelUp);
    }
}
