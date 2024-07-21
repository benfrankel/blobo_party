use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use pyri_state::prelude::*;

use crate::core::UpdateSet;
use crate::game::card::CardKey;
use crate::game::card::CardStorage;
use crate::game::step::on_step;
use crate::screen::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        Screen::Playing.on_update((execute_queued_cards
            .in_set(UpdateSet::Update)
            .run_if(on_step(4)),)),
    );
}

#[derive(Component)]
pub struct Deck {
    pub cards: Vec<CardKey>,
    next_card: usize,
}

impl Deck {
    fn peak_next(&self) -> Option<&CardKey> {
        self.cards.get(self.next_card)
    }

    fn rotate(&mut self) {
        if !self.cards.is_empty() {
            self.next_card = (self.next_card + 1) % self.cards.len();
        }
    }
}

pub fn create_deck(mut entity: EntityWorldMut) {
    entity.insert(Deck {
        cards: vec![
            CardKey::Placeholder,
            CardKey::Placeholder,
            CardKey::Placeholder,
        ],
        next_card: 0,
    });
}

pub fn execute_queued_cards(world: &mut World) {
    let mut system_state: SystemState<(Res<CardStorage>, Query<(Entity, &Deck)>)> =
        SystemState::new(world);
    let (card_storage, decks) = system_state.get(world);

    // Grab all the queued cards along with their Entity
    let queued_cards = decks
        .iter()
        .filter_map(|(e, d)| {
            d.peak_next()
                .map(|card_key| (e, card_storage[card_key].action))
        })
        .collect::<Vec<_>>();

    // Execute the queued action for each deck
    for (entity, card_action_system_id) in queued_cards {
        world
            .run_system_with_input(card_action_system_id, entity)
            .unwrap();
    }

    // Rotate each deck
    for mut deck in world.query::<&mut Deck>().iter_mut(world) {
        deck.rotate();
    }
}
