use bevy::prelude::*;

use crate::core::UpdateSet;
use crate::game::level::xp::Xp;
use crate::game::level::Level;
use crate::game::level::LevelConfig;
use crate::util::prelude::*;

// TODO: System that enters level up menu on LevelUp event.
pub(super) fn plugin(app: &mut App) {
    app.configure::<LevelUp>();
}

/// A buffered event sent when an actor levels up.
#[derive(Event)]
pub struct LevelUp(#[allow(unused)] Entity);

impl Configure for LevelUp {
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
    mut level_query: Query<(&mut Level, &mut Xp)>,
) {
    let config = r!(config.get());
    if config.levels.is_empty() {
        return;
    }

    for (mut level, mut xp) in &mut level_query {
        loop {
            let level_cost = config.level(level.current + level.up).xp_cost;
            if xp.relative < level_cost {
                break;
            }

            xp.relative -= level_cost;
            level.up += 1;
        }
    }
}

fn trigger_level_up(
    mut level_up_events: EventWriter<LevelUp>,
    mut level_query: Query<(Entity, &mut Level)>,
) {
    for (entity, mut level) in &mut level_query {
        if level.up <= 0 {
            continue;
        }

        level.up -= 1;
        level.current += 1;
        level_up_events.send(LevelUp(entity));
    }
}
