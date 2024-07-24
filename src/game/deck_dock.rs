use bevy::prelude::*;
use leafwing_input_manager::common_conditions::action_just_pressed;
use pyri_state::prelude::*;
use rand::seq::SliceRandom;
use serde::Deserialize;
use serde::Serialize;
use strum::IntoEnumIterator;

use crate::game::actor::player::IsPlayer;
use crate::game::card::AddCardEvent;
use crate::game::card::CardConfig;
use crate::game::card::CardKey;
use crate::game::card::CardStorage;
use crate::game::deck::Deck;
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
            handle_added_cards,
            highlight_selected,
            add_card.run_if(action_just_pressed(PlayingAction::AddCard)),
        )),
    );
}

#[derive(Asset, Reflect, Serialize, Deserialize)]
pub struct DeckDockConfig {
    pub max_card_count: usize,
    pub px_between_cards: f32,
    pub card_highlight_offset: f32,
    pub dock_bottom_padding: f32,
    pub dock_height: f32,
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
                justify_content: JustifyContent::Center,
                column_gap: Val::Px(config.px_between_cards),
                padding: UiRect {
                    bottom: Val::Px(config.dock_bottom_padding),
                    ..default()
                },
                ..default()
            },
            ..default()
        },
        DeckDockMarker,
    ));
}

#[derive(Component)]
struct DeckDockMarker;

#[derive(Component)]
struct VisualCardMarker;

fn add_card(mut added_card_event_writer: EventWriter<AddCardEvent>) {
    let card_keys = CardKey::iter().collect::<Vec<_>>();
    let random_card = card_keys.choose(&mut rand::thread_rng());
    if let Some(random_card) = random_card {
        added_card_event_writer.send(AddCardEvent(*random_card));
    }
}

fn highlight_selected(
    deck_dock: Query<&Children, With<DeckDockMarker>>,
    player_deck: Query<&Deck, With<IsPlayer>>,
    config_handle: Res<ConfigHandle<DeckDockConfig>>,
    config: Res<Assets<DeckDockConfig>>,
    mut visual_cards: Query<&mut Style, With<VisualCardMarker>>,
) {
    if let (Ok(deck), Ok(children)) = (player_deck.get_single(), deck_dock.get_single()) {
        let config = r!(config.get(&config_handle.0));
        for (i, child) in children.iter().enumerate() {
            if let Ok(mut style) = visual_cards.get_mut(*child) {
                if i == deck.selected() {
                    style.top = Val::Px(config.card_highlight_offset);
                } else {
                    style.top = Val::Px(0.0);
                }
            }
        }
    }
}

fn handle_added_cards(
    mut commands: Commands,
    mut added_card_event_reader: EventReader<AddCardEvent>,
    dock: Query<Entity, With<DeckDockMarker>>,
) {
    for event in added_card_event_reader.read() {
        for dock_entity in &dock {
            commands.entity(dock_entity).with_children(|children| {
                let card = event.0;
                children.spawn_with(move |e: EntityWorldMut| visual_card(e, card));
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
    let card_storage = entity.world().resource::<CardStorage>();
    let card = &card_storage[&card_key];

    entity
        .insert((
            ImageBundle {
                style: Style {
                    position_type: PositionType::Relative,
                    ..default()
                },
                image: UiImage::new(config.card_backgrounds[&card.color].texture.clone()),
                ..default()
            },
            VisualCardMarker,
        ))
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

fn exit_playing(mut commands: Commands, dock: Query<Entity, With<DeckDockMarker>>) {
    for entity in &dock {
        if let Some(e) = commands.get_entity(entity) {
            e.despawn_recursive();
        }
    }
}
