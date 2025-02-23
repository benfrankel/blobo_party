use bevy::ecs::system::EntityCommand;
use bevy::ecs::system::SystemState;
use bevy::prelude::*;

use crate::core::UpdateSet;
use crate::game::actor::faction::Faction;
use crate::game::audio::AudioConfig;
use crate::game::audio::music::on_beat;
use crate::game::card::CardConfig;
use crate::game::card::OnPlayCard;
use crate::game::card::action::CardActionKey;
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
        app.add_observer(count_kills);
        app.add_observer(count_played_cards);
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
            (stats.beats as f64 / 8.0 * 60.0 / audio_config.music_bpm
                + audio_config.music_zeroth_beat)
                .floor()
                .to_string(),
            "seconds partied".to_string(),
            stats.kills.to_string(),
            "blobos impressed".to_string(),
            stats.played_moves.to_string(),
            "dances performed".to_string(),
            stats.played_attacks.to_string(),
            "notes played".to_string(),
            stats.played_heals.to_string(),
            "rests taken".to_string(),
        ];

        world
            .entity_mut(id)
            .insert((
                Name::new("StatsGrid"),
                Node {
                    display: Display::Grid,
                    grid_template_columns: RepeatedGridTrack::auto(2),
                    row_gap: Vw(1.2),
                    column_gap: Vw(2.5),
                    ..default()
                },
            ))
            .with_children(|children| {
                for (i, text) in stats.into_iter().enumerate() {
                    let (font, theme_color, justify_self) = if i % 2 == 0 {
                        (BOLD_FONT_HANDLE, ThemeColor::Indicator, JustifySelf::End)
                    } else {
                        (FONT_HANDLE, ThemeColor::BodyText, JustifySelf::Start)
                    };

                    children.spawn((
                        Name::new(format!("StatsSpan{}", i)),
                        Text::new(text),
                        TextFont::from_font(font),
                        theme_color.target::<TextColor>(),
                        DynamicFontSize::new(Vw(3.0)).with_step(8.0),
                        Node {
                            justify_self,
                            ..default()
                        },
                    ));
                }
            });
    }
}
