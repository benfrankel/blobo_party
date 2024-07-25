use bevy::ecs::system::SystemId;
use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::Deserialize;
use serde::Serialize;
use strum::EnumIter;

use crate::util::prelude::*;

mod movement;
mod remove;

pub(super) fn plugin(app: &mut App) {
    app.configure::<ConfigHandle<CardConfig>>()
        .register_type::<CardKey>()
        .add_plugins((movement::plugin,))
        .add_event::<AddCardEvent>();
}

#[derive(Event)]
pub struct AddCardEvent(pub CardKey);

#[derive(Asset, Reflect, Serialize, Deserialize)]
pub struct CardConfig {
    cards: HashMap<CardKey, CardInfo>,
    pub card_backgrounds: HashMap<CardColor, CardBackground>,
}

#[derive(Reflect, Eq, PartialEq, Hash, Copy, Clone, Serialize, Deserialize, EnumIter)]
pub enum CardColor {
    Yellow,
    Blue,
    Green,
    Magenta,
}

#[derive(Asset, Reflect, Serialize, Deserialize)]
pub struct CardBackground {
    texture_path: String,
    #[serde(skip)]
    pub texture: Handle<Image>,
}

#[derive(Asset, Reflect, Serialize, Deserialize)]
struct CardInfo {
    name: String,
    description: String,
    texture_path: String,
    card_color: CardColor,
    #[serde(skip)]
    texture: Handle<Image>,
}

impl Config for CardConfig {
    const PATH: &'static str = "config/card.ron";
    const EXTENSION: &'static str = "card.ron";

    fn on_load(&mut self, world: &mut World) {
        let mut system_state = SystemState::<Res<AssetServer>>::new(world);
        let asset_server = system_state.get_mut(world);

        for card_background in self.card_backgrounds.values_mut() {
            card_background.texture = asset_server.load(&card_background.texture_path);
        }

        for card in self.cards.values_mut() {
            card.texture = asset_server.load(&card.texture_path);
        }

        let cards = self.cards.iter().map(|(key, value)| {
            (
                *key,
                Card {
                    display_name: value.name.clone(),
                    description: value.description.clone(),
                    action: get_system_id(world, key),
                    color: value.card_color,
                    texture: value.texture.clone(),
                },
            )
        });

        let card_storage = CardStorage(cards.collect());
        world.insert_resource(card_storage);
    }

    fn is_ready(&self, asset_server: &AssetServer) -> bool {
        self.card_backgrounds
            .values()
            .all(|x| asset_server.is_loaded_with_dependencies(&x.texture))
            && self
                .cards
                .values()
                .all(|x| asset_server.is_loaded_with_dependencies(&x.texture))
    }
}

// TODO: This works for mapping Cards to their Actions but it might
// be better in another file and maybe as a resource?
fn get_system_id(world: &mut World, card: &CardKey) -> SystemId<Entity> {
    let action = match card {
        CardKey::BasicStep => basic_step,
        _ => noop,
    };

    world.register_system(action)
}

fn basic_step(In(entity): In<Entity>, world: &mut World) {
    if let Some(mut e) = world.get_entity_mut(entity) {
        e.insert((
            movement::Move,
            remove::RemoveOnBeat::<movement::Move>::new(5),
        ));
    }
}

fn noop(In(_): In<Entity>, _world: &mut World) {}

#[allow(dead_code)]
pub struct Card {
    pub display_name: String,
    pub description: String,
    pub texture: Handle<Image>,
    pub action: SystemId<Entity>,
    pub color: CardColor,
}

#[derive(Reflect, Eq, PartialEq, Hash, Copy, Clone, Serialize, Deserialize, EnumIter)]
pub enum CardKey {
    BasicStep,
    DoubleBeat,
    CounterPoint,
    Splits,
}

#[derive(Resource, Deref, DerefMut)]
pub struct CardStorage(pub HashMap<CardKey, Card>);
