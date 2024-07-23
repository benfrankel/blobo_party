use bevy::prelude::*;
use leafwing_input_manager::common_conditions::action_just_pressed;
use pyri_state::prelude::*;
use rand::seq::SliceRandom;
use serde::Deserialize;
use serde::Serialize;
use strum::IntoEnumIterator;

use crate::core::UpdateSet;
use crate::game::actor::player::IsPlayer;
use crate::game::card::AddCardEvent;
use crate::game::card::CardConfig;
use crate::game::card::CardKey;
use crate::game::card::CardStorage;
use crate::game::deck::Deck;
use crate::game::music::beat::on_beat;
use crate::screen::playing::PlayingAction;
use crate::screen::Screen;
use crate::ui::prelude::*;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<ConfigHandle<DeckDockConfig>>();

    app.add_systems(
        StateFlush,
        Screen::Playing.on_edge(exit_playing, enter_playing),
    );

    app.add_systems(
        Update,
        Screen::Playing.on_update((
            animate_move_towards,
            handle_added_cards,
            rotate_dock_left
                .in_set(UpdateSet::Update)
                .run_if(on_beat(2)),
            rotate_dock_left.run_if(action_just_pressed(PlayingAction::RotateDock)),
            add_card.run_if(action_just_pressed(PlayingAction::AddCard)),
        )),
    );
}

#[derive(Asset, Reflect, Serialize, Deserialize)]
pub struct DeckDockConfig {
    pub visible_card_count: usize,
    pub minimum_card_distance: usize,
    pub card_width: usize,
    pub dock_height: f32,
    pub min_card_scale: f32,
    pub max_card_scale: f32,
}

impl Config for DeckDockConfig {
    const PATH: &'static str = "config/deck_dock.ron";

    const EXTENSION: &'static str = "deck_dock.ron";

    fn on_load(&mut self, _: &mut World) {}
}

fn enter_playing(mut commands: Commands, ui_root: Res<UiRoot>) {
    commands.spawn_with(deck_dock).set_parent(ui_root.body);
}

fn deck_dock(mut entity: EntityWorldMut) {
    let config_handle = entity.world().resource::<ConfigHandle<DeckDockConfig>>();
    let config = r!(entity
        .world()
        .resource::<Assets<DeckDockConfig>>()
        .get(&config_handle.0),);

    entity.insert((
        Name::new("DeckDock"),
        NodeBundle {
            style: Style {
                width: Percent(100.0),
                height: Percent(config.dock_height),
                align_self: AlignSelf::End,
                align_items: AlignItems::End,
                ..default()
            },
            ..default()
        },
        DeckDockMarker,
    ));
}

#[derive(Component)]
struct DeckDockMarker;

#[derive(Component, Deref, DerefMut)]
struct CardPosition(usize);

fn add_card(
    mut added_card_event_writer: EventWriter<AddCardEvent>,
    player_deck: Query<&Deck, With<IsPlayer>>,
) {
    for deck in &player_deck {
        let count = deck.cards.len();
        let card_keys = CardKey::iter().collect::<Vec<_>>();
        let random_card = card_keys.choose(&mut rand::thread_rng());
        if let Some(random_card) = random_card {
            added_card_event_writer.send(AddCardEvent {
                card: *random_card,
                index: count,
            });
        }
    }
}

fn handle_added_cards(
    mut commands: Commands,
    mut added_card_event_reader: EventReader<AddCardEvent>,
    dock: Query<Entity, With<DeckDockMarker>>,
    mut card_positions: Query<&mut CardPosition>,
) {
    for event in added_card_event_reader.read() {
        for mut card_position in &mut card_positions {
            if **card_position >= event.index {
                **card_position += 1;
            }
        }
        for dock_entity in &dock {
            commands.entity(dock_entity).with_children(|children| {
                let card = event.card;
                children
                    .spawn_with(move |e: EntityWorldMut| visual_card(e, card))
                    .insert(CardPosition(event.index));
            });
        }
    }
}

fn visual_card(mut entity: EntityWorldMut, card_key: CardKey) {
    let config_handle = entity.world().resource::<ConfigHandle<CardConfig>>();
    let config = r!(entity
        .world()
        .resource::<Assets<CardConfig>>()
        .get(&config_handle.0),);

    entity
        .insert((ImageBundle {
            style: Style {
                position_type: PositionType::Absolute,
                ..default()
            },
            transform: Transform::from_scale(Vec3::splat(0.1)),
            image: UiImage::new(config.card_texture.clone()),
            ..default()
        },))
        .with_children(|children| {
            children.spawn_with(move |e: EntityWorldMut| add_icon(e, card_key));
        });
}

fn add_icon(mut entity: EntityWorldMut, card_key: CardKey) {
    let card_storage = entity.world().resource::<CardStorage>();
    let card = &card_storage[&card_key];

    entity.insert((ImageBundle {
        style: Style {
            position_type: PositionType::Relative,
            ..default()
        },
        transform: Transform::from_scale(Vec3::splat(0.5)),
        image: UiImage::new(card.texture.clone()),
        ..default()
    },));
}

#[derive(Component)]
struct MoveToward {
    start: CardDisplay,
    end: CardDisplay,
    duration: Timer,
}

fn lerp(start: f32, end: f32, fraction: f32) -> f32 {
    start + fraction * (end - start)
}

fn animate_move_towards(
    mut commands: Commands,
    mut move_towards: Query<(Entity, &mut Style, &mut Transform, &mut MoveToward)>,
    time: Res<Time>,
) {
    for (entity, mut style, mut transform, mut move_toward) in &mut move_towards {
        move_toward.duration.tick(time.delta());
        let fraction = move_toward.duration.fraction();

        transform.scale = move_toward
            .start
            .scale
            .lerp(move_toward.end.scale, fraction);
        style.left = Val::Percent(lerp(move_toward.start.left, move_toward.end.left, fraction));
        style.bottom = Val::Percent(lerp(
            move_toward.start.bottom,
            move_toward.end.bottom,
            fraction,
        ));

        if let (true, Some(mut e)) = (move_toward.duration.finished(), commands.get_entity(entity))
        {
            e.remove::<MoveToward>();
        }
    }
}

fn exit_playing(mut commands: Commands, dock: Query<Entity, With<DeckDockMarker>>) {
    for entity in &dock {
        if let Some(e) = commands.get_entity(entity) {
            e.despawn_recursive();
        }
    }
}

struct CardDisplay {
    left: f32,
    bottom: f32,
    scale: Vec3,
}

fn rotate_dock_left(
    mut commands: Commands,
    config_handle: Res<ConfigHandle<DeckDockConfig>>,
    config: Res<Assets<DeckDockConfig>>,
    mut cards: Query<(Entity, &Style, &Transform, &mut CardPosition)>,
) {
    let config = r!(config.get(&config_handle.0));
    let number_of_cards = cards.iter().len();
    for (entity, style, transform, mut card_position) in &mut cards {
        **card_position = if **card_position == 0 {
            number_of_cards - 1
        } else {
            **card_position - 1
        };

        let end = get_display_information(number_of_cards, config, **card_position);

        commands.entity(entity).insert((MoveToward {
            start: CardDisplay {
                left: if let Val::Percent(left) = style.left {
                    left
                } else {
                    0.0
                },
                bottom: if let Val::Percent(bottom) = style.bottom {
                    bottom
                } else {
                    0.0
                },
                scale: transform.scale,
            },
            end,
            duration: Timer::from_seconds(0.5, TimerMode::Once),
        },));
    }
}

fn get_display_information(
    number_of_cards: usize,
    config: &DeckDockConfig,
    index: usize,
) -> CardDisplay {
    let DeckDockConfig {
        visible_card_count,
        minimum_card_distance,
        card_width,
        min_card_scale,
        max_card_scale,
        ..
    } = *config;

    let total_width = number_of_cards * card_width + (number_of_cards - 1) * minimum_card_distance;
    let visible_width = if visible_card_count > number_of_cards {
        total_width
    } else {
        visible_card_count * card_width + (visible_card_count - 1) * minimum_card_distance
    };
    let item_width = card_width + minimum_card_distance;

    let center_index = if number_of_cards % 2 == 0 {
        number_of_cards / 2 - 1
    } else {
        (number_of_cards - 1) / 2
    };
    let x_position = (index as isize - center_index as isize) * item_width as isize;
    let left_percentage = if number_of_cards == 2 && index != 0 {
        20.0 // handling edge case when there are only two cards
    } else {
        x_position as f32 / visible_width as f32 * 100.0
    };

    let left = left_percentage + 50.0;

    let height_step = 100.0 / number_of_cards as f32;
    let bottom = if index <= center_index {
        100.0 - (center_index - index) as f32 * height_step
    } else {
        100.0 - (index - center_index) as f32 * height_step
    };

    let max_distance = visible_card_count as f32;
    let distance_from_center = (index as isize - center_index as isize).abs() as f32;
    let scale = lerp(
        min_card_scale,
        max_card_scale,
        1.0 - (distance_from_center / max_distance),
    );

    CardDisplay {
        left,
        bottom,
        scale: Vec3::splat(scale),
    }
}
