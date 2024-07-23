use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use pyri_state::prelude::*;

use crate::core::UpdateSet;
use crate::game::actor::player::IsPlayer;
use crate::game::card::AddCardEvent;
use crate::game::card::CardKey;
use crate::game::card::CardStorage;
use crate::game::music::beat::on_beat;
use crate::screen::Screen;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Deck>();
    app.add_systems(
        Update,
        Screen::Playing.on_update((
            handle_player_added_cards,
            execute_queued_cards
                .in_set(UpdateSet::Update)
                .run_if(resource_exists::<CardStorage>.and_then(on_beat(2))),
        )),
    );
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
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

fn handle_player_added_cards(
    mut added_card_event_reader: EventReader<AddCardEvent>,
    mut player_deck: Query<&mut Deck, With<IsPlayer>>,
) {
    for event in added_card_event_reader.read() {
        for mut deck in &mut player_deck {
            deck.cards.insert(event.index, event.card);
        }
    }
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
