use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::core::UpdateSet;
use crate::game::audio::music::on_full_beat;
use crate::game::card::card;
use crate::game::card::CardConfig;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(Deck, IsDeckDisplay)>();
}

#[derive(Component, Reflect, Serialize, Deserialize, Default, Clone)]
#[reflect(Component)]
pub struct Deck {
    #[serde(rename = "cards")]
    pub card_keys: Vec<String>,
    #[serde(default)]
    pub active: usize,
}

impl Configure for Deck {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            play_card_from_deck
                .in_set(UpdateSet::PlayCards)
                .run_if(on_full_beat(2)),
        );
    }
}

impl Deck {
    pub fn new(cards: impl Into<Vec<String>>) -> Self {
        Self {
            card_keys: cards.into(),
            active: 0,
        }
    }

    pub fn advance(&mut self, step: isize) -> Option<&String> {
        if self.card_keys.is_empty() {
            return None;
        }

        self.active =
            (self.active as isize + step).rem_euclid(self.card_keys.len() as isize) as usize;

        Some(&self.card_keys[self.active])
    }

    pub fn swap(&mut self, step: isize) {
        if self.card_keys.is_empty() {
            return;
        }

        let old = self.active;
        self.active =
            (self.active as isize + step).rem_euclid(self.card_keys.len() as isize) as usize;
        self.card_keys.swap(old, self.active);
    }

    pub fn discard(&mut self) {
        if self.card_keys.len() <= 1 {
            return;
        }

        self.card_keys.remove(self.active);
        if self.active >= self.card_keys.len() {
            self.active = 0;
        }
    }

    pub fn add(&mut self, card_key: impl Into<String>) {
        self.card_keys.insert(self.active, card_key.into());
    }
}

fn play_card_from_deck(
    mut commands: Commands,
    config: ConfigRef<CardConfig>,
    mut deck_query: Query<(Entity, &mut Deck)>,
) {
    let config = r!(config.get());

    for (entity, mut deck) in &mut deck_query {
        let card_key = c!(deck.advance(1));
        let card_action = c!(config.card_map.get(card_key));
        let action = card_action.action;
        let action_config = card_action.action_config.clone();
        commands.run_system_with_input(action.0, (entity, action_config));
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct IsDeckDisplay;

impl Configure for IsDeckDisplay {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            (
                clear_deck_display.in_set(UpdateSet::Despawn),
                populate_deck_display.in_set(UpdateSet::Spawn),
            ),
        );
    }
}

/// Clear deck display on any change.
fn clear_deck_display(
    mut commands: Commands,
    deck_display_query: Query<(Entity, &Selection), With<IsDeckDisplay>>,
    target_changed_query: Query<(), Changed<Selection>>,
    deck_changed_query: Query<(), Changed<Deck>>,
) {
    for (entity, selection) in &deck_display_query {
        if !target_changed_query.contains(entity) && !deck_changed_query.contains(selection.0) {
            continue;
        }

        commands.entity(entity).despawn_descendants();
    }
}

/// Populate deck display on any change.
fn populate_deck_display(
    mut commands: Commands,
    deck_display_query: Query<(Entity, &Selection), With<IsDeckDisplay>>,
    deck_query: Query<&Deck>,
    target_changed_query: Query<(), Changed<IsDeckDisplay>>,
    deck_changed_query: Query<(), Changed<Deck>>,
) {
    for (entity, selection) in &deck_display_query {
        if !target_changed_query.contains(entity) && !deck_changed_query.contains(selection.0) {
            continue;
        }
        let deck = deck_query.get(selection.0).unwrap();

        commands.entity(entity).with_children(|children| {
            for (i, card_key) in deck.card_keys.iter().enumerate() {
                children.spawn_with(card(card_key, Some(i == deck.active)));
            }
        });
    }
}
