use bevy::ecs::system::EntityCommand;
use bevy::ecs::system::SystemState;
use bevy::prelude::*;

use crate::core::UpdateSet;
use crate::game::actor::faction::Faction;
use crate::game::audio::music::on_beat;
use crate::game::audio::AudioConfig;
use crate::game::card::action::CardActionKey;
use crate::game::card::CardConfig;
use crate::game::card::OnPlayCard;
use crate::game::combat::death::OnDeath;
use crate::ui::prelude::*;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<Stats>();
}

#[derive(Resource, Reflect, Default, Copy, Clone)]
#[reflect(Resource)]
pub struct Stats {
    pub beats: usize,
    pub kills: usize,
    pub played_moves: usize,
    pub played_attacks: usize,
    pub played_heals: usize,
}

impl Configure for Stats {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_resource::<Self>();
        app.add_systems(
            Update,
            count_beats.in_set(UpdateSet::Update).run_if(on_beat(1)),
        );
        app.observe(count_kills);
        app.observe(count_played_cards);
    }
}

fn count_beats(mut stats: ResMut<Stats>) {
    stats.beats += 1;
}

fn count_kills(
    trigger: Trigger<OnDeath>,
    faction_query: Query<&Faction>,
    mut stats: ResMut<Stats>,
) {
    let entity = r!(trigger.get_entity());
    let faction = r!(faction_query.get(entity));
    if !faction.is_enemy() {
        return;
    }

    stats.kills += 1;
}

fn count_played_cards(
    trigger: Trigger<OnPlayCard>,
    config: ConfigRef<CardConfig>,
    faction_query: Query<&Faction>,
    mut stats: ResMut<Stats>,
) {
    let entity = r!(trigger.get_entity());
    let faction = r!(faction_query.get(entity));
    if !faction.is_player() {
        return;
    }

    let config = r!(config.get());
    let card = r!(config.card_map.get(&trigger.event().0));
    match card.action_key {
        CardActionKey::Attack => stats.played_attacks += 1,
        CardActionKey::Heal => stats.played_heals += 1,
        _ => stats.played_moves += 1,
    }
}

impl EntityCommand for Stats {
    fn apply(self, id: Entity, world: &mut World) {
        let mut system_state = SystemState::<(ConfigRef<AudioConfig>, Res<Stats>)>::new(world);
        let (audio_config, stats) = system_state.get(world);
        let audio_config = r!(audio_config.get());
        let stats = [
            format!(
                "[b]{:.0}",
                stats.beats as f64 / 8.0 * 60.0 / audio_config.music_bpm
                    + audio_config.music_zeroth_beat
            ),
            "seconds partied".to_string(),
            format!("[b]{}", stats.kills),
            "blobos impressed".to_string(),
            format!("[b]{}", stats.played_moves),
            "dances performed".to_string(),
            format!("[b]{}", stats.played_attacks),
            "notes played".to_string(),
            format!("[b]{}", stats.played_heals),
            "rests taken".to_string(),
        ];

        world
            .entity_mut(id)
            .insert((
                Name::new("StatsGrid"),
                NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        grid_template_columns: RepeatedGridTrack::auto(2),
                        row_gap: Vw(1.2),
                        column_gap: Vw(2.5),
                        ..default()
                    },
                    ..default()
                },
            ))
            .with_children(|children| {
                for (i, text) in stats.into_iter().enumerate() {
                    children.spawn((
                        Name::new("StatsSpan"),
                        TextBundle::from_sections(parse_rich(&text)).with_style(Style {
                            justify_self: if i % 2 == 0 {
                                JustifySelf::End
                            } else {
                                JustifySelf::Start
                            },
                            ..default()
                        }),
                        DynamicFontSize::new(Vw(3.0)).with_step(8.0),
                        ThemeColorForText(vec![if i % 2 == 0 {
                            ThemeColor::Indicator
                        } else {
                            ThemeColor::BodyText
                        }]),
                    ));
                }
            });
    }
}
