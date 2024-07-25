use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use super::CardConfig;
use crate::core::UpdateSet;
use crate::game::actor::player::IsPlayer;
use crate::game::card::AddCardEvent;
use crate::game::music::beat::on_beat;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<Deck>();
}

#[derive(Component, Reflect, Serialize, Deserialize, Default, Clone)]
#[reflect(Component)]
pub struct Deck {
    pub cards: Vec<String>,
    #[serde(default)]
    pub selected: usize,
}

impl Configure for Deck {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            (
                add_cards_to_deck.in_set(UpdateSet::SyncLate),
                play_cards.in_set(UpdateSet::PlayCards).run_if(on_beat(2)),
            ),
        );
    }
}

impl Deck {
    pub fn new(cards: impl Into<Vec<String>>) -> Self {
        Self {
            cards: cards.into(),
            selected: 0,
        }
    }

    fn peek_next(&self) -> Option<&String> {
        self.cards.get(self.next())
    }

    fn next(&self) -> usize {
        if !self.cards.is_empty() {
            (self.selected + 1) % self.cards.len()
        } else {
            0
        }
    }

    fn advance(&mut self) {
        if !self.cards.is_empty() {
            self.selected = self.next();
        }
    }
}

fn add_cards_to_deck(
    mut add_card_events: EventReader<AddCardEvent>,
    mut player_deck: Query<&mut Deck, With<IsPlayer>>,
) {
    for event in add_card_events.read() {
        for mut deck in &mut player_deck {
            deck.cards.push(event.0.clone());
        }
    }
}

fn play_cards(
    mut commands: Commands,
    config: ConfigRef<CardConfig>,
    mut deck_query: Query<(Entity, &mut Deck)>,
) {
    let config = r!(config.get());

    for (entity, mut deck) in &mut deck_query {
        let card_key = c!(deck.peek_next());
        let action = c!(config.card_map.get(card_key)).action;
        commands.run_system_with_input(action.0, entity);
        deck.advance();
    }
}
