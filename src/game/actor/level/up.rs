use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use pyri_state::prelude::*;

use crate::core::UpdateSet;
use crate::game::actor::health::Health;
use crate::game::actor::level::xp::Xp;
use crate::game::actor::level::Level;
use crate::game::actor::level::LevelConfig;
use crate::game::actor::player::IsPlayer;
use crate::screen::playing::victory_menu::EndlessMode;
use crate::screen::playing::PlayingAssets;
use crate::screen::playing::PlayingMenu;
use crate::util::prelude::*;

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
                heal_on_level_up
                    .in_set(UpdateSet::Update)
                    .run_if(on_event::<Self>()),
                play_level_up_sfx
                    .in_set(UpdateSet::Update)
                    .run_if(on_event::<Self>()),
                update_level_up_from_xp.in_set(UpdateSet::TriggerLevelUp),
                trigger_level_up
                    .in_set(UpdateSet::TriggerLevelUp)
                    .run_if(PlayingMenu::will_be_disabled),
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
    endless_mode: Res<EndlessMode>,
    mut playing_menu: NextMut<PlayingMenu>,
    audio: Res<Audio>,
    assets: Res<PlayingAssets>,
) {
    for (entity, mut level) in &mut level_query {
        if level.up == 0 {
            continue;
        }

        if !endless_mode.0 && level.current + 1 == 10 {
            playing_menu.enter(PlayingMenu::Victory);
            audio.play(assets.sfx_level_up.clone()).with_volume(0.8);
        } else {
            level.up -= 1;
            level.current += 1;
            level_up_events.send(LevelUp(entity));
        }
    }
}

fn play_level_up_sfx(audio: Res<Audio>, assets: Res<PlayingAssets>) {
    audio.play(assets.sfx_level_up.clone()).with_volume(0.8);
}

fn heal_on_level_up(mut player_query: Query<&mut Health, With<IsPlayer>>) {
    for mut health in &mut player_query {
        health.max += 15.0;
        health.current = health.max;
    }
}
