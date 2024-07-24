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
    pub gap_between_cards: Val,
    pub card_highlight_offset: Val,
    pub height: Val,
}

impl Config for DeckDockConfig {
    const PATH: &'static str = "config/deck_dock.ron";
    const EXTENSION: &'static str = "deck_dock.ron";

    fn on_load(&mut self, _: &mut World) {}
}

pub fn deck_dock(mut entity: EntityWorldMut) {
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
                height: config.height,
                justify_content: JustifyContent::Center,
                column_gap: config.gap_between_cards,
                ..default()
            },
            ..default()
        },
        IsDeckDock,
    ));
}

#[derive(Component)]
struct IsDeckDock;

#[derive(Component)]
struct IsVisualCard;

fn add_card(mut add_card_events: EventWriter<AddCardEvent>) {
    let card_keys = CardKey::iter().collect::<Vec<_>>();
    let random_card = card_keys.choose(&mut rand::thread_rng());
    if let Some(random_card) = random_card {
        add_card_events.send(AddCardEvent(*random_card));
    }
}

fn highlight_selected(
    deck_dock: Query<&Children, With<IsDeckDock>>,
    player_deck: Query<&Deck, With<IsPlayer>>,
    config_handle: Res<ConfigHandle<DeckDockConfig>>,
    config: Res<Assets<DeckDockConfig>>,
    mut visual_cards: Query<&mut Style, With<IsVisualCard>>,
) {
    if let (Ok(deck), Ok(children)) = (player_deck.get_single(), deck_dock.get_single()) {
        let config = r!(config.get(&config_handle.0));
        for (i, child) in children.iter().enumerate() {
            if let Ok(mut style) = visual_cards.get_mut(*child) {
                if i == deck.selected() {
                    style.top = config.card_highlight_offset;
                } else {
                    style.top = Px(0.0);
                }
            }
        }
    }
}

fn handle_added_cards(
    mut commands: Commands,
    mut added_card_event_reader: EventReader<AddCardEvent>,
    dock: Query<Entity, With<IsDeckDock>>,
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
            IsVisualCard,
        ))
        .with_children(|children| {
            children.spawn_with(move |e: EntityWorldMut| add_icon(e, card_key));
        });
}

fn add_icon(mut entity: EntityWorldMut, card_key: CardKey) {
    let card_storage = entity.world().resource::<CardStorage>();
    let card = &card_storage[&card_key];

    entity.insert(ImageBundle {
        style: Style {
            position_type: PositionType::Relative,
            ..default()
        },
        transform: Transform::from_scale(Vec3::splat(0.5)),
        image: UiImage::new(card.texture.clone()),
        ..default()
    });
}
