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
    app.init_resource::<SelectedIndex>();

    app.add_systems(
        Update,
        Screen::Playing.on_update((
            handle_added_cards,
            highlight_selected,
            set_selected_from_player_deck,
            add_card.run_if(action_just_pressed(PlayingAction::AddCard)),
            // TODO: Run these on level up screen
            swap_card_left.run_if(action_just_pressed(PlayingAction::SwapCardLeft)),
            swap_card_right.run_if(action_just_pressed(PlayingAction::SwapCardRight)),
            select_left.run_if(action_just_pressed(PlayingAction::SelectCardLeft)),
            select_right.run_if(action_just_pressed(PlayingAction::SelectCardRight)),
            // TODO: Run this when exiting level up screen
            sync_to_player_deck.run_if(action_just_pressed(PlayingAction::AcceptDeckChanges)),
        )),
    );
}

#[derive(Default, Resource, Deref, DerefMut)]
struct SelectedIndex(usize);

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

    fn on_load(&mut self, world: &mut World) {
        for mut style in world
            .query_filtered::<&mut Style, With<IsDeckDock>>()
            .iter_mut(world)
        {
            style.height = self.height;
            style.column_gap = self.gap_between_cards;
        }
    }
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
struct VisualCard(CardKey);

fn add_card(mut add_card_events: EventWriter<AddCardEvent>) {
    let card_keys = CardKey::iter().collect::<Vec<_>>();
    let random_card = card_keys.choose(&mut rand::thread_rng());
    if let Some(random_card) = random_card {
        add_card_events.send(AddCardEvent(*random_card));
    }
}

fn set_selected_from_player_deck(
    player_deck: Query<&Deck, With<IsPlayer>>,
    mut selected_index: ResMut<SelectedIndex>,
) {
    if let Some(deck) = player_deck.iter().last() {
        **selected_index = deck.selected();
    }
}

fn highlight_selected(
    deck_dock: Query<&Children, With<IsDeckDock>>,
    selected_index: Res<SelectedIndex>,
    config_handle: Res<ConfigHandle<DeckDockConfig>>,
    config: Res<Assets<DeckDockConfig>>,
    mut visual_cards: Query<&mut Style, With<VisualCard>>,
) {
    if let Some(children) = deck_dock.iter().last() {
        let config = r!(config.get(&config_handle.0));
        for (i, child) in children.iter().enumerate() {
            if let Ok(mut style) = visual_cards.get_mut(*child) {
                if i == **selected_index {
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
            VisualCard(card_key),
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

fn swap_card_left(
    mut selected_index: ResMut<SelectedIndex>,
    mut deck_dock: Query<&mut Children, With<IsDeckDock>>,
) {
    if **selected_index == 0 {
        // TODO: play a bad sound
        return;
    }

    let Some(mut children) = deck_dock.iter_mut().last() else {
        return;
    };

    children.swap(**selected_index, **selected_index - 1);
    **selected_index = selected_index.saturating_sub(1);
}

fn swap_card_right(
    mut selected_index: ResMut<SelectedIndex>,
    mut deck_dock: Query<&mut Children, With<IsDeckDock>>,
) {
    let Some(mut children) = deck_dock.iter_mut().last() else {
        return;
    };
    if **selected_index >= children.len() - 1 {
        // TODO: play a bad sound
        return;
    }

    children.swap(**selected_index, **selected_index + 1);
    **selected_index = selected_index.saturating_add(1);
}

fn select_left(mut selected_index: ResMut<SelectedIndex>) {
    **selected_index = selected_index.saturating_sub(1);
}
fn select_right(
    mut selected_index: ResMut<SelectedIndex>,
    deck_dock: Query<&Children, With<IsDeckDock>>,
) {
    let Some(children) = deck_dock.iter().last() else {
        return;
    };

    **selected_index = selected_index.saturating_add(1).min(children.len() - 1);
}

fn sync_to_player_deck(
    mut player_deck: Query<&mut Deck, With<IsPlayer>>,
    deck_dock: Query<&Children, With<IsDeckDock>>,
    visual_cards: Query<&VisualCard>,
) {
    let Some(children) = deck_dock.iter().last() else {
        return;
    };

    let cards = children
        .iter()
        .filter_map(|child| visual_cards.get(*child).ok())
        .map(|card| card.0)
        .collect::<Vec<_>>();

    for mut deck in &mut player_deck {
        *deck = Deck::new(cards.clone())
    }
}
