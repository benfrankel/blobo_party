use bevy::ecs::system::SystemId;
use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::Deserialize;
use serde::Serialize;
use strum::EnumIter;

use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<ConfigHandle<CardConfig>>()
        .add_event::<AddCardEvent>();
}

#[derive(Event)]
pub struct AddCardEvent {
    pub card: CardKey,
    pub index: usize,
}

#[derive(Asset, Reflect, Serialize, Deserialize)]
pub struct CardConfig {
    pub card_texture_path: String,
    #[serde(skip)]
    pub card_texture: Handle<Image>,
    cards: HashMap<CardKey, CardInfo>,
}

#[derive(Asset, Reflect, Serialize, Deserialize)]
struct CardInfo {
    name: String,
    description: String,
    texture_path: String,
    #[serde(skip)]
    texture: Handle<Image>,
}

impl Config for CardConfig {
    const PATH: &'static str = "config/card.ron";
    const EXTENSION: &'static str = "card.ron";

    fn on_load(&mut self, world: &mut World) {
        let id = world.register_system(basic_attack);
        let mut system_state = SystemState::<Res<AssetServer>>::new(world);
        let asset_server = system_state.get_mut(world);

        self.card_texture = asset_server.load(&self.card_texture_path);
        for card in self.cards.values_mut() {
            card.texture = asset_server.load(&card.texture_path);
        }

        let cards = self.cards.iter().map(|(key, value)| {
            (
                *key,
                Card {
                    display_name: value.name.clone(),
                    description: value.description.clone(),
                    action: id,
                    texture: value.texture.clone(),
                },
            )
        });

        world.insert_resource(CardStorage(cards.collect()));
    }

    fn is_ready(&self, asset_server: &AssetServer) -> bool {
        asset_server.is_loaded_with_dependencies(&self.card_texture)
            && self
                .cards
                .values()
                .all(|x| asset_server.is_loaded_with_dependencies(&x.texture))
    }
}

#[allow(dead_code)]
pub struct Card {
    pub display_name: String,
    pub description: String,
    pub texture: Handle<Image>,
    pub action: SystemId<Entity>,
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

// TODO: Move this into a sub-folder of storing different attacks?
fn basic_attack(In(_): In<Entity>) {}
